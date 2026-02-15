use std::{
    collections::VecDeque,
    io::{stdin, ErrorKind, Read},
    pin::Pin,
    task::{ready, Poll},
    thread,
};

use tokio::{
    io::{AsyncRead, ReadBuf},
    sync::mpsc::{channel, Receiver},
};
use tokio_util::sync::PollSender;

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
    available: Option<(Box<[u8]>, usize, usize)>,
    pending_reads: VecDeque<usize>,
    pending_total: usize,
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
            available: None,
            pending_reads: VecDeque::new(),
            pending_total: 0,
        }
    }

    fn fill_from_available(&mut self, buf: &mut ReadBuf<'_>) -> bool {
        match self.available.take() {
            Some((avail, start, len)) => {
                let to_return = std::cmp::min(len, buf.remaining());
                buf.put_slice(&avail[start..start + to_return]);

                if to_return < len {
                    self.available = Some((avail, start + to_return, len - to_return));
                }
                true
            }
            None => false,
        }
    }
}

impl AsyncRead for Stdin {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // Return any data that we already have available
        if self.fill_from_available(buf) {
            return Poll::Ready(Ok(()));
        }

        // Backpressure: don't request more bytes if we have outstanding read requests that can
        //               account for what is being requested here.
        if self.pending_total < buf.remaining() {
            let reserve_result = ready!(self.in_sender.poll_reserve(cx));
            match reserve_result {
                Ok(_) => {
                    // Send input, transition state, then continue
                    if let Err(_) = self.in_sender.send_item(buf.remaining()) {
                        return Poll::Ready(Err(std::io::Error::new(
                            ErrorKind::BrokenPipe,
                            "Communication error while trying to read from stdin; pipe is potentially closed",
                        )))
                    }
                    self.pending_reads.push_back(buf.remaining());
                    self.pending_total += buf.remaining();
                },
                Err(_) => return Poll::Ready(Err(std::io::Error::new(
                    ErrorKind::BrokenPipe,
                    "Communication error while trying to read from stdin; pipe is potentially closed",
                ))),
            }
        }

        // Read the result back from the reader thread via the receiver
        let read_result = ready!(self.out_receiver.poll_recv(cx));
        let req_len = self
            .pending_reads
            .pop_front()
            .expect("Unreachable: We cannot receive more results than we requested");
        self.pending_total -= req_len;
        match read_result {
            Some(result) => match result {
                Ok(data) => {
                    let len = data.len();
                    if len > 0 {
                        self.available = Some((data, 0, len));
                        self.fill_from_available(buf);
                    }
                    Poll::Ready(Ok(()))
                }
                Err(e) => Poll::Ready(Err(e)),
            },
            None => Poll::Ready(Ok(())),
        }
    }
}
