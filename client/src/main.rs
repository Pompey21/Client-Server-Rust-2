// ======================================
// SERVER IMPLEMENTATION
// ======================================

use std::io::{Read, Write};
use std::net::{TcpStream};
use std::thread;
mod user;
use user::User;



// fn main() {
//     let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//     let message = b"Hello, server!";
//     let mut num_message: i64 = 42;
//     let num_message_str = num_message.to_string();

//     // send message to server
//     stream.write_all(&num_message.to_be_bytes()).unwrap();
//     stream.write_all(&num_message_str.as_bytes()).unwrap();

//     // receive response from server
//     let mut buffer = [0u8; 8];
//     stream.read(&mut buffer).unwrap();
//     let received_num_message = i64::from_be_bytes(buffer);
//     println!("Received: {}\n",received_num_message);
// }

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut user_x = User::new("Marko".to_string(), "123".to_string(), 100, 100, false);




    let num_message: i32 = 42;
    let num_message_str = num_message.to_string();

    // send message to server
    // stream.write_all(&num_message.to_be_bytes()).unwrap();
    stream.write_all(&num_message_str.as_bytes()).unwrap();

    // receive response from server
    let mut buffer = [0u8; 8];
    stream.read(&mut buffer).unwrap();

    let received_num_message_str: String = String::from_utf8_lossy(&buffer).into_owned();

    // let received_num_message = i32::from_be_bytes(buffer);
    println!("Received: {}\n",received_num_message_str);

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
