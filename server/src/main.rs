// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream}, collections::HashMap, sync::{Arc, RwLock}};

mod user;
use user::User;

fn handle_serialised_user_object(mut stream: TcpStream, user_log: Arc<RwLock<HashMap<User, bool>>>) {

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


                
                // Modify the global variable from the main task
                let mut write_lock = user_log.write().unwrap();
                write_lock.insert(received_user_object, true);

                // break;


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