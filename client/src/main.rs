// ======================================
// CLIENT IMPLEMENTATION
// ======================================

use std::io::{Read, Write};
use std::net::{TcpStream};
use std::thread;
mod user;
use user::User;
// use tokio::sync::{RwLock, Mutex};
// use tokio::task;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut user_x = User::new("Marko".to_string(), "123".to_string(), 100, 100, false);
    let my_serialized_user_x = bincode::serialize(&user_x).unwrap();
    println!("{my_serialized_user_x:?}");

    let my_deserialized_user_x = bincode::deserialize::<User>(&my_serialized_user_x).unwrap();
    println!("{my_deserialized_user_x:?}");

    // send message to server
    stream.write_all(&my_serialized_user_x).unwrap();

    // receive response from server
    const SIZE_OF_USER: usize = std::mem::size_of::<User>();
    let mut buffer = [0; SIZE_OF_USER];
    stream.read(&mut buffer).unwrap();

    let returned_message: User = bincode::deserialize::<User>(&buffer).unwrap();//String::from_utf8_lossy(&buffer).into_owned();

    // let received_num_message = i32::from_be_bytes(buffer);
    println!("Received: {:?}\n", returned_message);

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
