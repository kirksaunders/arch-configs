use interprocess::local_socket::LocalSocketStream;
use std::{
    error::Error,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = LocalSocketStream::connect("@testing")?;
    conn.write_all(b"Hello from client!\n")?;

    let mut conn = BufReader::new(conn);
    let mut buffer = String::new();
    conn.read_line(&mut buffer)?;

    println!("Server answered: {}", buffer);

    Ok(())
}
