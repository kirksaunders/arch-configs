use std::ffi::OsStr;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::Duration;

use futures::{StreamExt, join};

use async_std::io::{BufReader, stdin};
use async_std::io::prelude::BufReadExt;
use async_std::sync::Mutex;
use async_std::task::{sleep, spawn};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CLI {
    /// Interval at which to run the command (in seconds). If not supplied, the command will only be run once at the beginning and once more each time a line of input is read from stdin.
    #[structopt(short = "i", long = "interval")]
    interval: Option<f64>,

    /// Arguments to pass to the command.
    #[structopt(subcommand)]
    command: SubCommand
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Command to run with its arguments.
    #[structopt(external_subcommand)]
    Sub(Vec<String>)
}

impl SubCommand {
    #[allow(dead_code)]
    fn get(self) -> Vec<String> {
        match self {
            SubCommand::Sub(v) => v
        }
    }

    fn as_ref(&self) -> &Vec<String> {
        match self {
            SubCommand::Sub(v) => v
        }
    }

    #[allow(dead_code)]
    fn as_mut(&mut self) -> &Vec<String> {
        match self {
            SubCommand::Sub(v) => v
        }
    }
}

struct ChildData {
    proc: Option<Child>
}

async fn run(command: &[impl AsRef<OsStr>], data: Arc<Mutex<ChildData>>, input: impl AsRef<str>) {
    let mut data = data.lock().await;
    
    if let Some(child) = &mut data.proc {
        // Wait for last run to finish before running again
        let _ = child.wait();
    }

    data.proc = Some(Command::new(command[0].as_ref())
        .args(command.iter().skip(1))
        .stdin(Stdio::piped())
        .spawn()
        .expect("Unable to spawn child process"));

    data.proc.as_mut().unwrap().stdin.take().unwrap()
        .write_all(input.as_ref().as_bytes())
        .expect("Unable to write to child process's stdin");
}

async fn timer_loop(cli: &CLI, data: Arc<Mutex<ChildData>>) {
    match cli.interval {
        Some(interval) => loop {
            let t = spawn(sleep(Duration::from_secs_f64(interval)));
            run(cli.command.as_ref(), data.clone(), "").await;

            t.await;
        },
        None => run(cli.command.as_ref(), data.clone(), "").await
    }
}

async fn read_write_loop(cli: &CLI, data: Arc<Mutex<ChildData>>) {
    let mut lines = BufReader::new(stdin()).lines();
    while let Some(line) = lines.next().await {
        let mut line = line.expect("Unable to read from stdin");
        line.push('\n');

        run(cli.command.as_ref(), data.clone(), line).await;
    }
}

#[async_std::main]
async fn main() {
    let cli = CLI::from_args();

    let data = Arc::new(Mutex::new(ChildData {
        proc: None
    }));

    join!(timer_loop(&cli, data.clone()), read_write_loop(&cli, data));
}
