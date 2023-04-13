use core::num;
#[allow(dead_code)]
use std::io::{Read, Write, Result};
use std::net::{TcpStream, SocketAddr};

fn send_number(addr: SocketAddr, number: i32) -> Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    let bytes = number.to_be_bytes();
    stream.write_all(&bytes)?;
    Ok(())
}


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let message = b"Hello, server!";
    let num_message = 123456789;

    // send message to server
    stream.write_all(message).unwrap();
    send_number(stream.peer_addr().unwrap(), num_message);

    // receive response from server
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer));
}

