// ======================================
// CLIENT IMPLEMENTATION
// ======================================

mod requests;
mod user;
mod request;

use std::io::{Read, Write};
use std::net::{TcpStream};
use std::thread;

use user::User;
// use request::Request;
use requests::{POST_User, POST_Offer, POST_Request, GET_Offer, GET_Request, Request, Either, Either_Request, Request_1};

// use requests::{POST_User, POST_Offer, POST_Request, GET_Offer GET_Request, Request, Either, Either_Request};

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
    // TODO: 
    // let request_x = Request::new("POST".to_string(), "USER".to_string(), "".to_string(), user_x.clone(), offer_x);
    let post_req_user = POST_User::new(user_x.clone());
    let post_req = POST_Request::new("POST".to_string(), Either::POST_User(post_req_user)); 
    // let request = Request::new("POST".to_string(), Either_Request::POST_Request(post_req));

    let request_1: Request_1<POST_Request> = Request_1::<POST_Request>::new("POST".to_string(), post_req);

    // send request to server
    // TODO: 
    send_post_request(request_1);



// Sending the second request
    // create a request object
    let offer_y: Offer = Offer::new(100, 10, 100);
    // let request_y = Request::new("GET".to_string(), "USER".to_string(), "".to_string(), user_x.clone(), offer_y);

    // send request to server
    // send_request(request_y);

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

// ======================================
// Send Over Requests
// ======================================
fn send_post_request(request: Request_1<POST_Request>) {
    const HEADER_SIZE: usize = 4;
    const SIZE_OF_REQUEST: usize = std::mem::size_of::<Request_1<POST_Request>>();

    // convert message length to a 4-byte array in network byte order
    let message_len = SIZE_OF_REQUEST as u32;
    let mut header = [0; HEADER_SIZE];
    header.copy_from_slice(&message_len.to_be_bytes());

    // send message to server
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let my_serialized_request = bincode::serialize(&request).unwrap();
    stream.write_all(&header).unwrap();
    stream.write_all(&my_serialized_request).unwrap();

    // receive response from server
    let mut buffer = [0; SIZE_OF_REQUEST];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]).to_string();
    println!("Response: {}", response);
}

fn send_get_request(request: Request_1<GET_Request>) {
    const HEADER_SIZE: usize = 4;
    const SIZE_OF_REQUEST: usize = std::mem::size_of::<Request_1<GET_Request>>();

    // convert message length to a 4-byte array in network byte order
    let message_len: u32 = SIZE_OF_REQUEST as u32;
    let mut header = [0; HEADER_SIZE];
    header.copy_from_slice(&message_len.to_be_bytes());

    // send message to server
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let my_serialized_request = bincode::serialize(&request).unwrap();
    stream.write_all(&header).unwrap();
    stream.write_all(&my_serialized_request).unwrap();

    // receive response from server
    let mut buffer = [0; SIZE_OF_REQUEST];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]);
    println!("Response: {}", response);
}
