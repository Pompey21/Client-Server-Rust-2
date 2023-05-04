// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream}, collections::HashMap};
mod user;
use user::User;

fn handle_serialised_user_object(mut stream: TcpStream) {
    const SIZE_OF_USER: usize = std::mem::size_of::<User>();
    let mut buffer = [0; SIZE_OF_USER];
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
                let received_user_object = bincode::deserialize::<User>(&buffer).unwrap();
                println!("Received: {:?}\n", received_user_object);

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
    // let mut user_log = HashMap::new();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and spawn a thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{:?}", stream.local_addr().unwrap());
                println!("handlam");
                std::thread::spawn(|| handle_serialised_user_object(stream));
                // handle_client_ultimate(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}\n", e);
            }
        }
    }
}