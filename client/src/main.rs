use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let message = b"Hello, server!";

    // send message to server
    stream.write_all(message).unwrap();

    // receive response from server
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer));
}

