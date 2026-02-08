use std::io::Write;
use std::io::{stdin, stdout, Read};
use std::thread::sleep;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Arguments {
    /// Delay (in seconds) between each animation frame
    #[structopt(short = "d", long = "delay", default_value = "0.01")]
    delay: f64,

    /// Step size (in characters) between each animation frame
    #[structopt(short = "s", long = "step", default_value = "1")]
    step: usize,

    /// Where to cut string (amount of characters) to make static (animation will occur past these characters)
    #[structopt(short = "c", long = "cut", default_value = "0")]
    cut: usize,

    /// Animation mode to run in
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(StructOpt, Debug)]
enum Mode {
    /// Animate forwards (string starts empty and grows in size)
    Forward,

    /// Animate in reverse (string starts full and shrinks in size)
    Reverse,
}

#[derive(Serialize, Deserialize)]
enum Content {
    Raw(String),
    Tag {
        prefix: String,
        suffix: String,
        content: Box<Content>,
    },
    Concatenation(Vec<Content>),
}

impl Content {
    fn len(&self) -> usize {
        match self {
            Content::Raw(str) => str.chars().count(),
            Content::Tag {
                prefix: _,
                suffix: _,
                content: con,
            } => con.len(),
            Content::Concatenation(content) => content.iter().fold(0, |acc, s| acc + s.len()),
        }
    }

    fn write(&self, out: &mut (impl Write + ?Sized), limit: usize) -> std::io::Result<usize> {
        if limit > 0 {
            match self {
                Content::Raw(str) => {
                    let chars: Vec<char> = str.chars().take(limit).collect();
                    let amnt = chars.len();
                    out.write_all(chars.into_iter().collect::<String>().as_bytes())?;
                    Ok(amnt)
                }
                Content::Tag {
                    prefix,
                    suffix,
                    content,
                } => {
                    out.write_all(prefix.as_bytes())?;
                    let amnt = content.write(out, limit)?;
                    out.write_all(suffix.as_bytes())?;
                    Ok(amnt)
                }
                Content::Concatenation(content) => {
                    let mut amnt = 0;
                    for c in content {
                        if limit - amnt == 0 {
                            break;
                        }

                        amnt += c.write(out, limit - amnt)?;
                    }
                    Ok(amnt)
                }
            }
        } else {
            Ok(0)
        }
    }
}

fn animate(content: Content, end_idx: usize, delay: f64, iter: impl Iterator<Item = usize>) {
    let mut last = 0;
    for i in iter {
        last = i;
        content.write(&mut stdout(), i).unwrap();
        println!();
        stdout().flush().unwrap();

        sleep(Duration::from_secs_f64(delay));
    }

    if last != end_idx {
        content.write(&mut stdout(), end_idx).unwrap();
        println!()
    }
}

fn main() {
    let args = Arguments::from_args();

    /*let content = Content::Concatenation(vec![
        Content::Tag {
            prefix: "$button_pre".to_string(),
            suffix: "$suffix".to_string(),
            content: Box::new(Content::Tag {
                prefix: "$button_font".to_string(),
                suffix: "%{T-}".to_string(),
                content: Box::new(Content::Raw("$button".to_string()))
            })
        },
        Content::Raw("$separator".to_string()),
        Content::Tag {
            prefix: "$logout_pre".to_string(),
            suffix: "$suffix".to_string(),
            content: Box::new(Content::Raw("$logout".to_string()))
        },
        Content::Raw("$separator".to_string()),
        Content::Tag {
            prefix: "$reboot_pre".to_string(),
            suffix: "$suffix".to_string(),
            content: Box::new(Content::Raw("$reboot".to_string()))
        },
        Content::Raw("$separator".to_string()),
        Content::Tag {
            prefix: "$shutdown_pre".to_string(),
            suffix: "$suffix".to_string(),
            content: Box::new(Content::Raw("$shutdown".to_string()))
        },
    ]);
    println!("{}", serde_json::to_string(&content).unwrap());*/

    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Unable to read from stdin");

    let content: Content = serde_json::from_str(&input).expect("Unable to parse input as json");

    let content_len = content.len();
    let base_iter = args.cut..=content_len;
    match args.mode {
        Mode::Forward => animate(content, content_len, args.delay, base_iter.step_by(args.step)),
        Mode::Reverse => animate(content, args.cut, args.delay, base_iter.rev().step_by(args.step)),
    };
}
