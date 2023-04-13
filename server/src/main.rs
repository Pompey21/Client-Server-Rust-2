#[allow(dead_code)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> i64 {
    let mut buffer = [0u8; 8];
    loop {
        match stream.read(&mut buffer) {
            // Ok(n) if n == 0 => {
            //     // connection closed
            //     break;
            // }
            Ok(n) => {
                // echo back to the client
                stream.write_all(&buffer[0..n]).unwrap();
                let received_num_message = i64::from_be_bytes(buffer);
                println!("Received: {}\n", received_num_message);
                return received_num_message;
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                // break;
                return -1;
            }
        }
    }
}



fn main() {
    let mut global_ticker: i64 = 0;
    print!("global_ticker: {}\n", global_ticker);

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and spawn a thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // why do I need to spawn a new thread? 
                // with out it, the server will only handle one connection!
                // std::thread::spawn(|| handle_client(stream));
                let received_number = handle_client(stream);
                global_ticker = received_number;
                print!("global_ticker: {}\n", global_ticker);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}\n", e);
            }
        }
    }
}