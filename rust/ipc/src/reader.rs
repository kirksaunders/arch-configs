use std::{
    io::{ErrorKind, Read, stdin},
    pin::Pin,
    task::{Poll, ready},
    thread,
};

use tokio::{
    io::{AsyncRead, ReadBuf},
    sync::mpsc::{channel, Receiver},
};
use tokio_util::sync::PollSender;

enum StdinState {
    PendingSend,
    PendingResult,
}

/// Tokio's stdin operates in a weird middle ground between blocking and non-blocking. Namely, when
/// the runtime is shutting down, any outstanding calls to stdin.read will continue to block, which
/// prevent the program from exiting. Ref: https://docs.rs/tokio/latest/tokio/io/struct.Stdin.html
/// 
/// This implementation does not have that limitation. It spins up a platform thread to handle reads
/// from stdin, then passes the read results back to the async context via channels. It should act
/// the exact same as Tokio's stdin otherwise. There are some performance implications however:
///   1) Read bytes are copied from the platform thread to the async context.
///   2) The reading and writing has to cross thread boundaries, which may add some latency.
///   3) Each instance of this struct will spin up its own platform thread.
pub struct Stdin {
    in_sender: PollSender<usize>,
    out_receiver: Receiver<std::io::Result<Box<[u8]>>>,
    state: StdinState,
}

impl Stdin {
    pub fn new(buffer_size: usize) -> Self {
        let (in_sender, mut in_receiver) = channel::<usize>(2);
        let (out_sender, out_receiver) = channel::<std::io::Result<Box<[u8]>>>(2);
        thread::spawn(move || {
            let mut buf = vec![0; buffer_size];
            let mut stdin = stdin();
            while let Some(num_bytes) = in_receiver.blocking_recv() {
                let result = match stdin.read(&mut buf[..std::cmp::min(num_bytes, buffer_size)]) {
                    Ok(num_read) => Ok(Box::from(&buf[..num_read])),
                    Err(e) => Err(e),
                };
                if let Err(_) = out_sender.blocking_send(result) {
                    break;
                }
            }
        });
        Self {
            in_sender: PollSender::new(in_sender),
            out_receiver,
            state: StdinState::PendingSend,
        }
    }
}

impl AsyncRead for Stdin {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();

        // First, send input to reader thread via the sender
        if let StdinState::PendingSend = this.state {
            let reserve_result = ready!(this.in_sender.poll_reserve(cx));
            match reserve_result {
                Ok(_) => {
                    // Send input, transition state, then continue
                    if let Err(_) = this.in_sender.send_item(buf.remaining()) {
                        return Poll::Ready(Err(std::io::Error::new(
                            ErrorKind::BrokenPipe,
                            "Communication error while trying to read from stdin; pipe is potentially closed",
                        )))
                    }
                    this.state = StdinState::PendingResult;
                },
                Err(_) => return Poll::Ready(Err(std::io::Error::new(
                    ErrorKind::BrokenPipe,
                    "Communication error while trying to read from stdin; pipe is potentially closed",
                ))),
            }
        }

        // Then, read the result back from the reader thread via the receiver
        let read_result = ready!(this.out_receiver.poll_recv(cx));
        let final_result = match read_result {
            Some(result) => match result {
                Ok(data) => {
                    buf.put_slice(&data);
                    Poll::Ready(Ok(()))
                }
                Err(e) => Poll::Ready(Err(e)),
            },
            None => Poll::Ready(Ok(())),
        };

        // Make sure we clear the state before returning
        this.state = StdinState::PendingSend;
        final_result
    }
}
