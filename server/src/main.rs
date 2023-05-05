// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream}, collections::HashMap, sync::{Arc, RwLock}};

mod request;
mod user;
use user::User;
use request::Request;

fn handle_post_request(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>) {
    if received_user_object.get_post_type().starts_with("USER") {
        let mut write_lock = user_log.write().unwrap();
        write_lock.insert(received_user_object.get_user_data().clone(), true);
        println!("User logged in: {:?}", write_lock);
    }

    if received_user_object.get_post_type().starts_with("OFFER") {
        println!("Offer type: {:?}", received_user_object.get_post_type());
    }
}

fn handle_serialised_user_object(mut stream: TcpStream, user_log: Arc<RwLock<HashMap<User, bool>>>) {

    const SIZE_OF_REQUEST: usize = std::mem::size_of::<Request>();
    let mut buffer = [0; SIZE_OF_REQUEST];
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
                let received_user_object: Request = bincode::deserialize::<Request>(&buffer).unwrap();
                println!("Received: {:?}\n", received_user_object);

                if received_user_object.get_request_type().starts_with("POST") {
                    handle_post_request(received_user_object, user_log.clone());
                } 


            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                // break;
            }
        }
    }
}



#[tokio::main]
async fn main() {
    // creating that global variable allowing for concurrent access (writes and reads)
    let user_log:Arc<RwLock<HashMap<User, bool>>> = Arc::new(RwLock::new(HashMap::new()));


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and spawn a thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{:?}", stream.local_addr().unwrap());
                println!("handlam");

                let user_log_clone = user_log.clone();
                std::thread::spawn(|| {handle_serialised_user_object(stream, user_log_clone)});

            }
            Err(e) => {
                eprintln!("Error accepting connection: {}\n", e);
            }
        }
    }
}