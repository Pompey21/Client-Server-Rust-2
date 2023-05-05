// ======================================
// CLIENT IMPLEMENTATION
// ======================================

use std::io::{Read, Write};
use std::net::{TcpStream};
use std::thread;
mod user;
use user::User;
mod request;
use request::Request;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // create a user object
    let user_x = User::new("Marko".to_string(), "123".to_string(), 100, 100, false);
    let my_serialized_user_x = bincode::serialize(&user_x).unwrap();
    println!("{my_serialized_user_x:?}");
    let my_deserialized_user_x = bincode::deserialize::<User>(&my_serialized_user_x).unwrap();
    println!("{my_deserialized_user_x:?}");

    // create a request object
    let request_x = Request::new("POST".to_string(), "USER".to_string(), "".to_string(), user_x);
    let my_serialized_request_x = bincode::serialize(&request_x).unwrap();
    stream.write_all(&my_serialized_request_x).unwrap();

    // send message to server
    // stream.write_all(&my_serialized_user_x).unwrap();

    // receive response from server
    const SIZE_OF_REQUEST: usize = std::mem::size_of::<Request>();
    let mut buffer = [0; SIZE_OF_REQUEST];
    stream.read(&mut buffer).unwrap();

    let returned_message: Request = bincode::deserialize::<Request>(&buffer).unwrap();
    println!("Received: {:?}\n", returned_message);

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
