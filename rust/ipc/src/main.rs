mod reader;
mod writer;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    FutureExt, StreamExt,
};
use structopt::StructOpt;
use tokio::{
    io::{stdin, stdout, AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    join,
    net::{UnixListener, UnixStream},
};

use tokio::{select, spawn, sync::Mutex, time::sleep};

use crate::reader::Stdin;
use crate::writer::{AsyncWriteInterceptor, AsyncWriteObserver};
use Mode::*;

#[derive(StructOpt, Debug)]
struct Arguments {
    /// Socket to use for IPC
    #[structopt(short = "s", long = "socket", default_value = "/tmp/ipc")]
    socket: PathBuf,

    /// Whether to pass stdin to stdout or not, regardless of mode.
    #[structopt(short = "p", long = "passthrough")]
    passthrough: bool,

    /// Whether to run in server or client mode.
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(StructOpt, Debug)]
enum Mode {
    /// Run in server mode (establishes the socket).
    Server {
        /// Delete the socket file if it already exists prior to establishing server socket.
        #[structopt(short = "d", long = "deletesock")]
        delete: bool,

        /// Number of lines of data to keep in memory that will be written to new client connections.
        #[structopt(short = "m", long = "memory", default_value = "0")]
        memory: usize,
    },

    /// Run in client mode (connects to existing socket).
    Client {
        /// Interval (in seconds) at which retry connection until one is established, must be positive and non-zero. If not provided, the connection will not be retried.
        #[structopt(short = "r", long = "retryinterval")]
        retry: Option<f64>,

        /// Quit the connection when input from stdin is exhausted.
        #[structopt(short = "q", long = "quitearly")]
        quit: bool,
    },
}

type OutputStream = Box<dyn AsyncWrite + Unpin + Send>;

struct Memory {
    buf: Vec<u8>,
    lines: Vec<Box<[u8]>>,
}

impl Memory {
    fn new(capacity: usize) -> Self {
        Self {
            buf: Vec::new(),
            lines: Vec::with_capacity(capacity),
        }
    }

    fn append_line(&mut self, line: Box<[u8]>) {
        if self.lines.len() >= self.lines.capacity() {
            self.lines.remove(0);
        }
        self.lines.push(line);
    }
}

impl AsyncWriteObserver for Memory {
    fn write(&mut self, data: &[u8]) {
        let mut lines = data.split_inclusive(|byte| *byte == '\n' as u8);

        while let Some(line) = lines.next() {
            if line.len() == 0 {
                continue;
            }

            if line[line.len() - 1] == '\n' as u8 {
                // Small optimization to avoid a copy if the held buf is empty
                if self.buf.len() > 0 {
                    self.buf.extend_from_slice(line);
                    self.append_line(Box::from(self.buf.as_slice()));
                    self.buf.clear();
                } else {
                    self.append_line(Box::from(line));
                }
            } else {
                // Just append to the buf
                self.buf.extend_from_slice(line);
            }
        }
    }
}

async fn write_and_flush(data: &String, stream: &mut OutputStream) -> std::io::Result<()> {
    stream.write_all(data.as_bytes()).await?;
    stream.flush().await
}

async fn forward_mutable(input: impl AsyncBufRead + Unpin, outputs: Arc<Mutex<Vec<OutputStream>>>) {
    let mut lines = input.lines();
    loop {
        let result = lines.next_line().await;
        if let Err(_) = result {
            // Treat any error as a pipe close
            return;
        }
        match result.unwrap() {
            Some(mut line) => {
                line.push('\n');

                let mut outputs = outputs.lock().await;
                let mut failed_indices = Vec::new();
                {
                    // Write to all outputs concurrently
                    let mut futures = FuturesOrdered::new();
                    for (i, output) in outputs.iter_mut().enumerate() {
                        // Process them in reverse order (via push_front), which will make removals easier
                        // after the fact
                        futures.push_front(
                            write_and_flush(&line, output).map(move |result| (i, result)),
                        );
                    }

                    while let Some(result) = futures.next().await {
                        if let (i, Err(_)) = result {
                            failed_indices.push(i);
                        }
                    }
                }

                // Remove any failed streams from the outputs vec so we don't continue to try to write to
                // them
                for i in failed_indices.into_iter() {
                    outputs.swap_remove(i);
                }
            }
            None => return,
        }
    }
}

async fn forward_immutable(
    input: impl AsyncBufRead + Unpin,
    mut outputs: Box<[OutputStream]>,
    quit_on_output_failures: bool,
) {
    let mut lines = input.lines();
    loop {
        let result = lines.next_line().await;
        if let Err(_) = result {
            // Treat any error as a pipe close
            return;
        }
        match result.unwrap() {
            Some(mut line) => {
                line.push('\n');

                // Write to all outputs concurrently
                let mut futures = FuturesUnordered::new();
                for output in outputs.iter_mut() {
                    futures.push(write_and_flush(&line, output));
                }
                while let Some(result) = futures.next().await {
                    if let Err(_) = result {
                        if quit_on_output_failures {
                            return;
                        }
                    }
                }
            }
            None => return,
        }
    }
}

async fn client(socket: impl AsRef<Path>, passthrough: bool, retry: Option<f64>, quit_early: bool) {
    let stream = loop {
        match UnixStream::connect(socket.as_ref()).await {
            Ok(stream) => break stream,
            Err(e) => {
                if let Some(delay) = retry {
                    assert!(delay > 0f64, "Retry interval must be greater than zero");
                    sleep(Duration::from_secs_f64(delay)).await;
                } else {
                    panic!("Unable to connect to socket: {}", e);
                }
            }
        }
    };
    let (stream_in, stream_out) = stream.into_split();

    // Forward stdin to the socket input (and stdout if passthrough is enabled)
    let into_socket = forward_immutable(
        BufReader::new(Stdin::new(512)),
        if passthrough {
            Box::new([Box::new(stdout()), Box::new(stream_out)])
        } else {
            Box::new([Box::new(stream_out)])
        },
        true, // If we fail to write to either stdout or the server, then we might as well quit
    );
    // Forward socket output to stdout
    let out_of_socket = forward_immutable(
        BufReader::new(stream_in),
        Box::new([Box::new(stdout())]),
        true,
    );

    // If we want to quit after stdin has closed, then we can simply use select. If we want to continue after stdin closes, we must spawn
    // the stdin future into the background, then await the out_of_socket future in the foreground. In both cases, the program quits if
    // the server exits.
    if quit_early {
        select! {
            _ = into_socket => {}
            _ = out_of_socket => {}
        }
    } else {
        spawn(into_socket);
        out_of_socket.await;
    }
}

async fn accept_connections(
    socket: impl AsRef<Path>,
    outputs: Arc<Mutex<Vec<OutputStream>>>,
    memory: Option<&'static Mutex<Memory>>,
) {
    let listener = UnixListener::bind(socket).expect("Unable to create socket");
    'outer: loop {
        if let Ok((stream, _)) = listener.accept().await {
            let (stream_in, mut stream_out) = stream.into_split();

            if let Some(memory) = memory {
                let memory = memory.lock().await;

                for line in memory.lines.iter() {
                    match stream_out.write_all(line).await {
                        Ok(_) => (),
                        _ => continue 'outer, // client connection already closed?
                    }
                }
            }

            {
                let mut outputs = outputs.lock().await;
                outputs.push(Box::new(stream_out));
            }

            // We need to spawn here, otherwise we'll never pick up a new client connection
            // until this one ends.
            //
            // TODO: After this forward function returns, the client connection is closed. It would be
            //       great to remove it from the outputs vec at that point, but it's pretty non-trivial.
            //       For now, we just wait for the next write to outputs for the stale streams to be
            //       removed.
            spawn(forward_immutable(
                BufReader::new(stream_in),
                Box::new([Box::new(stdout())]),
                true,
            ));
        }
    }
}

async fn server(socket: impl AsRef<Path>, passthrough: bool, delete: bool, memory_size: usize) {
    if delete {
        let _ = std::fs::remove_file(socket.as_ref());
    }

    let mut outputs: Vec<OutputStream> = Vec::new();
    if passthrough {
        outputs.push(Box::new(stdout()));
    }
    let memory = if memory_size > 0 {
        let memory: &'static Mutex<Memory> =
            Box::leak(Box::new(Mutex::new(Memory::new(memory_size))));
        outputs.push(Box::new(AsyncWriteInterceptor::new(&memory)));
        Some(memory)
    } else {
        None
    };

    let outputs = Arc::new(Mutex::new(outputs));
    join!(
        forward_mutable(BufReader::new(stdin()), outputs.clone()),
        accept_connections(socket, outputs, memory)
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Arguments::from_args();

    match args.mode {
        Server { delete, memory } => server(args.socket, args.passthrough, delete, memory).await,
        Client { retry, quit } => client(args.socket, args.passthrough, retry, quit).await,
    };
}
