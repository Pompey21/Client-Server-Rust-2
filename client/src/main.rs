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
mod offer;
use offer::Offer;

#[tokio::main]
async fn main() {

// Sending the first request
    // create a user object
    let user_x = User::new("Marko".to_string(), "123".to_string(), 100, 100, false);
    // create an offer object
    let offer_x = Offer::new(100, 10, 100);
    // create a request object
    let request_x = Request::new("POST".to_string(), "USER".to_string(), "".to_string(), user_x.clone(), offer_x);

    // send request to server
    send_request(request_x);



// Sending the second request
    // create a request object
    let offer_y: Offer = Offer::new(100, 10, 100);
    let request_y = Request::new("GET".to_string(), "USER".to_string(), "".to_string(), user_x.clone(), offer_y);

    // send request to server
    send_request(request_y);

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

// ======================================
// Send Over Requests
// ======================================
fn send_request(request: Request) {
    // send message to server
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let my_serialized_request = bincode::serialize(&request).unwrap();
    stream.write_all(&my_serialized_request).unwrap();

    // receive response from server
    const SIZE_OF_REQUEST: usize = std::mem::size_of::<Request>();
    let mut buffer = [0; SIZE_OF_REQUEST];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]);
    println!("Response: {}", response);
}
