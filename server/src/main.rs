// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream, SocketAddr}, mem};
mod user;
use user::User;

fn handle_client_ultimate(mut stream: TcpStream) {
    let mut buffer = [0u8; 8];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => {
                // connection closed
                break;
            }
            Ok(n) => {
                println!("n: {}", n);
                // echo back to the client
                stream.write_all(&buffer[0..n]).unwrap();

                // handle the received message
                let received_num_message_str = String::from_utf8_lossy(&buffer).into_owned();
                println!("This is the string message: {}", received_num_message_str);
                println!("{}",received_num_message_str.len());
                let received_num_message_str_trimmed = received_num_message_str.trim_end_matches(char::from(0));
                println!("{}",received_num_message_str_trimmed.len());


                let num_message: i32 = received_num_message_str_trimmed.parse().unwrap();
                println!("Received: {}\n", num_message);
                // break;


            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                // break;
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
                println!("{:?}", stream.local_addr().unwrap());
                println!("handlam");
                std::thread::spawn(|| handle_client_ultimate(stream));
                // handle_client_ultimate(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}\n", e);
            }
        }
    }
}