use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("监听于 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buf = [0; 512];
        let bytes = stream.read(&mut buf)?;
        stream.write_all(&buf[..bytes])?; // 回显
    }
    Ok(())
}

#[allow(unused)]
fn connect_with_retry(addr: &str, retries: u32) -> io::Result<TcpStream> {
    let mut attempts = 0;
    loop {
        match TcpStream::connect(addr) {
            Ok(stream) => return Ok(stream),
            Err(e) if e.kind() == io::ErrorKind::ConnectionRefused && attempts < retries => {
                attempts += 1;
                sleep(Duration::from_secs(1));
            }
            Err(e) => return Err(e),
        }
    }
}

fn main() {
    println!("Hello, world!");
    listen().unwrap();
}
