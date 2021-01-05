use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub fn serve(address: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|e| log::error!("{:?}", e));
        });
    }
}

fn handler(mut stream: TcpStream) -> anyhow::Result<()> {
    log::debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            log::debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..n])?);
        stream.write_all(&buffer[..n])?;
    }
}
