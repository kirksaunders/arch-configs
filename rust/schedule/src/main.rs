use std::ffi::OsStr;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use structopt::StructOpt;
use tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::MissedTickBehavior;
use tokio::{join, time};

#[derive(StructOpt, Debug)]
struct CLI {
    /// Interval at which to run the command (in seconds). If not supplied, the command will only be run once at the beginning and once more each time a line of input is read from stdin.
    #[structopt(short = "i", long = "interval")]
    interval: Option<f64>,

    /// Arguments to pass to the command.
    #[structopt(subcommand)]
    command: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Command to run with its arguments.
    #[structopt(external_subcommand)]
    Sub(Vec<String>),
}

impl SubCommand {
    #[allow(dead_code)]
    fn get(self) -> Vec<String> {
        match self {
            SubCommand::Sub(v) => v,
        }
    }

    fn as_ref(&self) -> &Vec<String> {
        match self {
            SubCommand::Sub(v) => v,
        }
    }

    #[allow(dead_code)]
    fn as_mut(&mut self) -> &Vec<String> {
        match self {
            SubCommand::Sub(v) => v,
        }
    }
}

struct ChildData {
    proc: Option<Child>,
}

async fn run(command: &[impl AsRef<OsStr>], data: Arc<Mutex<ChildData>>, input: impl AsRef<str>) {
    let mut data = data.lock().await;

    if let Some(child) = &mut data.proc {
        // Wait for last run to finish before running again
        let _ = child.wait().await;
    }

    let mut proc = Command::new(command[0].as_ref())
        .args(command.iter().skip(1))
        .stdin(Stdio::piped())
        .spawn()
        .expect("Unable to spawn child process");

    let result = proc
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_ref().as_bytes())
        .await;
    if let Err(e) = result {
        eprintln!("Failed to write to child process stdin: {:?}", e);
    }

    data.proc = Some(proc);
}

async fn timer_loop(cli: &CLI, data: Arc<Mutex<ChildData>>) {
    match cli.interval {
        Some(interval) => {
            let mut ticker = time::interval(Duration::from_secs_f64(interval));
            ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
            loop {
                ticker.tick().await;
                run(cli.command.as_ref(), data.clone(), "").await;
            }
        }
        None => run(cli.command.as_ref(), data.clone(), "").await,
    }
}

async fn read_write_loop(cli: &CLI, data: Arc<Mutex<ChildData>>) {
    let mut lines = BufReader::new(stdin()).lines();
    while let Some(mut line) = lines.next_line().await.expect("Unable to read from stdin") {
        line.push('\n');

        run(cli.command.as_ref(), data.clone(), line).await;
    }
}

#[tokio::main]
async fn main() {
    let cli = CLI::from_args();
    let data = Arc::new(Mutex::new(ChildData { proc: None }));

    join!(timer_loop(&cli, data.clone()), read_write_loop(&cli, data));
}
