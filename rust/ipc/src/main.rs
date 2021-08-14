use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
//use std::process::{Command, Stdio};

use async_std::task;
use async_std::task::sleep;
use async_std::io::{stdin, stdout};
use async_std::sync::Mutex;

use futures::future::{AbortHandle, Abortable, Aborted, join};
use futures::io::BufReader;
use futures::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Future, StreamExt};

use tokio::net::{UnixListener, UnixStream};

use async_compat::{Compat, CompatExt};

use async_ctrlc::CtrlC;

use structopt::StructOpt;

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
    mode: Mode
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
        memory: usize
    },

    /// Run in client mode (connects to existing socket).
    Client {
        /// Interval (in seconds) at which retry connection until one is established, must be positive and non-zero. If not provided, the connection will not be retried.
        #[structopt(short = "r", long = "retryinterval")]
        retry: Option<f64>,

        /// Quit the connection when input from stdin is exhausted.
        #[structopt(short = "q", long = "quitearly")]
        quit: bool
    }
}

/*
#[derive(StructOpt, Debug)]
enum Action {
    /// Schedule a command to be run periodically or when a message is received on the given socket.
    Schedule {
        /// Regular interval to run command (in seconds). If not supplied, the command will only be run once at the start and then when a message appears on the socket.
        #[structopt(short = "i", long = "interval")]
        interval: Option<f64>,

        /// Whether to wait for the previous run of the command before running again or not.
        #[structopt(short = "w", long = "wait")]
        wait: bool,

        /// Command to run (encapsulate this as a string).
        command: String
    }
}

fn schedule(args: &Arguments) {
    let listener = block_on(LocalSocketListener::bind(args.socket.clone()))
            .expect("Unable to open socket for listening");
    
    let server = listener.incoming().try_for_each(|conn| async move {
        use futures::stream::StreamExt;

        let mut cmd = match &args.action {
            Notify => panic!("This shouldn't happen"),
            Schedule { interval: _, command } => Command::new("/bin/bash")
                .arg("-c")
                .arg(command)
                .stdin(Stdio::piped())
                .spawn()
                .expect("Unable to run command")
        };

        let mut outstdin = cmd.stdin.take().expect("Unable to open stdin for command");

        let mut lines = BufReader::new(conn).lines();
        while let Some(line) = lines.next().await {
            let mut line = line.expect("Unable to read from socket connection");
            line.push('\n');

            // Only write if command is still running
            if let Ok(None) = cmd.try_wait() {
                outstdin.write_all(line.as_bytes()).expect("Unable to write to stdin for command");
            }
        }

        Ok(())
    });

    block_on(server).expect("Unable to start server listener loop");
}*/

struct StreamData {
    output: Vec<Box<dyn AsyncWrite + Unpin + Send>>,
    memory: Vec<String>
}

type StreamVec = Vec<Box<dyn AsyncWrite + Unpin + Send>>;

async fn forward<I: AsyncRead + Unpin>(input: I, data: Arc<Mutex<StreamData>>) {
    let mut lines = BufReader::new(input).lines();
    while let Some(line) = lines.next().await {
        let mut line = line.expect("Unable to read from socket");
        line.push('\n');

        let mut data = data.lock().await;

        let mut i = 0;
        while i < data.output.len() {
            match data.output[i].write_all(line.as_bytes()).await {
                Ok(_) => i += 1,
                _ => { data.output.swap_remove(i); }
            }
        }

        if data.memory.capacity() > 0 {
            if data.memory.len() == data.memory.capacity() {
                data.memory.remove(0);
            }
            data.memory.push(line);
        }
    }
}

async fn listen(socket: impl AsRef<Path>, data: Arc<Mutex<StreamData>>) {
    let listener = UnixListener::bind(socket).expect("Unable to create socket");

    'outer: loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let stream = stream.compat();
                let (mut stream_in, mut stream_out) = stream.split();
        
                {
                    let mut data = data.lock().await;

                    for line in data.memory.iter() {
                        match stream_out.write_all(line.as_bytes()).await {
                            Ok(_) => (),
                            _ => break 'outer // client connection already closed?
                        }
                    }

                    data.output.push(Box::new(stream_out));
                }

                let data2 = data.clone();
                task::spawn(async move {
                    async_std::io::copy(&mut stream_in, &mut stdout()).await.expect("Unable to copy from socket to stdout");

                    // At this point the connection has closed, so we should remove it from the output vector.
                    // However, since we don't know which element in the vector refers to this connection, let's
                    // just cleanup the whole thing.
                    let mut data = data2.lock().await;

                    let mut i = 0;
                    while i < data.output.len() {
                        match data.output[i].write(&[]).await {
                            Ok(_) => i += 1,
                            _ => { data.output.swap_remove(i); }
                        }
                    }
                });
            },
            Err(e) => eprintln!("Unable to establish client connection: {}", e)
        }
    }
}

async fn wrap_ctrlc<F, T>(fut: F) -> Result<T, Aborted>
where
    F: Future<Output = T> + Send,
    T: Send
{
    let (abort_handle1, abort_registration1) = AbortHandle::new_pair();
    let (abort_handle2, abort_registration2) = AbortHandle::new_pair();
    let ctrlc = CtrlC::new().expect("Unable to create ctrl+c hook");

    task::spawn(Abortable::new(async move {
        ctrlc.await;
        abort_handle1.abort();
    }, abort_registration2));

    Abortable::new(Compat::new(async move {
        let ret = fut.await;
        abort_handle2.abort();
        ret
    }), abort_registration1).await
}

async fn server(socket: impl AsRef<Path>, passthrough: bool, delete: bool, memory: usize) {
    if delete {
        let _ = std::fs::remove_file(socket.as_ref());
    }
    
    let mut data = StreamData {
        output: Vec::new(),
        memory: Vec::with_capacity(memory)
    };

    if passthrough {
        data.output.push(Box::new(stdout()));
    }

    let data = Arc::new(Mutex::new(data));

    let forwarder = forward(stdin(), data.clone());
    let listener = listen(socket.as_ref(), data);

    let _ = wrap_ctrlc(join(forwarder, listener)).await;

    let _ = std::fs::remove_file(socket.as_ref());
}

async fn connect(socket: impl AsRef<Path>, output: StreamVec, retry: Option<f64>, quit: bool) {
    let mut output = output;
    let stream = loop {
        match UnixStream::connect(socket.as_ref()).await {
            Ok(stream) => break stream.compat(),
            Err(e) => if let Some(delay) = retry {
                assert!(delay > 0f64, "Retry interval must be greater than zero");
                sleep(Duration::from_secs_f64(delay)).await;
            } else {
                panic!("Unable to connect to socket: {}", e);
            }
        }
    };

    let (mut stream_in, stream_out) = stream.split();

    output.push(Box::new(stream_out));

    let (abort_handle1, abort_registration1) = AbortHandle::new_pair();
    let (abort_handle2, abort_registration2) = AbortHandle::new_pair();

    task::spawn(Abortable::new(async move {
        let mut lines = BufReader::new(stdin()).lines();
        while let Some(line) = lines.next().await {
            let mut line = line.expect("Unable to read from socket");
            line.push('\n');
    
            for out in output.iter_mut() {
                out.write_all(line.as_bytes()).await.expect("Unable to write to output");
            }
        }

        // Quit the program if quitearly is on and we have read all input from stdin
        if quit {
            abort_handle2.abort();
        }
    }, abort_registration1));

    let _ = Abortable::new(async move {
        async_std::io::copy(&mut stream_in, &mut stdout()).await.expect("Unable to copy from socket to stdout");

        // We only reach this point if the above line finishes (meaning the connection has been broken)
        abort_handle1.abort();
    }, abort_registration2).await;
}

async fn client(socket: impl AsRef<Path>, passthrough: bool, retry: Option<f64>, quit: bool) {
    let mut output: StreamVec = Vec::new();
    if passthrough {
        output.push(Box::new(stdout()));
    }

    let _ = wrap_ctrlc(connect(socket.as_ref(), output, retry, quit)).await;
}

#[async_std::main]
async fn main() {
    let args = Arguments::from_args();

    match args.mode {
        Server { delete, memory } => server(args.socket, args.passthrough, delete, memory).await,
        Client { retry, quit } => client(args.socket, args.passthrough, retry, quit).await
    };
}
