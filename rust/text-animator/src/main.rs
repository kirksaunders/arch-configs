use std::env;
use std::io::Write;
use std::io::{Read, stdin, stdout};
use std::thread::sleep;
use std::time::Duration;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Content {
    Raw(String),
    Tag {
        prefix: String,
        suffix: String,
        content: Box<Content>
    },
    Concatenation(Vec<Content>)
}

impl Content {
    fn len(&self) -> usize {
        match self {
            Content::Raw(str) => str.chars().count(),
            Content::Tag{prefix: _, suffix: _, content: con} => con.len(),
            Content::Concatenation(content) => content.iter().
                    fold(0, |acc, s| acc + s.len())
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
                },
                Content::Tag{prefix, suffix, content} => {
                    out.write_all(prefix.as_bytes())?;
                    let amnt = content.write(out, limit)?;
                    out.write_all(suffix.as_bytes())?;
                    Ok(amnt)
                },
                Content::Concatenation(content) => {
                    let mut limit = limit;
                    let mut amnt = 0;
                    for c in content {
                        if limit == 0 {
                            break;
                        }

                        let a = c.write(out, limit)?;
                        amnt += a;
                        limit -= a;
                    }
                    Ok(amnt)
                }
            }
        } else {
            Ok(0)
        }
    }
}

enum Mode {
    FORWARD,
    REVERSE
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut delay = 0.01;
    let mut step = 1;
    let mut cut = 0;
    let mut mode = Mode::FORWARD;

    for arg in &args[1..] {
        match &arg[..3] {
            "-d=" => delay = arg[3..].parse::<f64>().expect("Invalid delay value"),
            "-s=" => step = arg[3..].parse::<usize>().expect("Invalid step value"),
            "-c=" => cut = arg[3..].parse::<usize>().expect("Invalid cut index value"),
            "-m=" => {
                match &arg[3..] {
                    "forward" => mode = Mode::FORWARD,
                    "reverse" => mode = Mode::REVERSE,
                    _ => panic!("Invalid mode")
                }
            }
            _ => panic!("Unknown argument: {}", arg)
        }
    }

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
    stdin().read_to_string(&mut input).expect("Unable to read from stdin");

    let content: Content = serde_json::from_str(&input).expect("Unable to parse input as json");

    match mode {
        Mode::FORWARD => {
            let mut last = 0;
            for i in (cut..=content.len()).step_by(step) {
                let t = std::time::Instant::now();
                last = i;
                content.write(&mut stdout(), i).unwrap();
                println!();
                stdout().flush().unwrap();

                sleep(Duration::from_secs_f64(delay));
                println!("{:?}", t.elapsed());
            }

            if last != content.len() {
                content.write(&mut stdout(), content.len()).unwrap();
                println!()
            }
        },
        Mode::REVERSE => {
            let mut last = 0;
            for i in (cut..=content.len()).rev().step_by(step) {
                last = i;
                content.write(&mut stdout(), i).unwrap();
                println!();
                stdout().flush().unwrap();

                sleep(Duration::from_secs_f64(delay));
            }

            if last != cut {
                content.write(&mut stdout(), cut).unwrap();
                println!()
            }
        }
    }
}
