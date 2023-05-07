// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream}, collections::HashMap, sync::{Arc, RwLock}};

mod request;
mod user;
mod offer;

use user::User;
use request::Request;
use offer::Offer;


fn handle_post_request(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    if received_user_object.get_post_type().starts_with("USER") {
        let mut write_lock = user_log.write().unwrap();
        write_lock.insert(received_user_object.get_user_data().clone(), true);
        println!("User logged in: {:?}", write_lock);
    }

    if received_user_object.get_post_type().starts_with("OFFER") {
        let mut write_lock = offers_log.write().unwrap();
        write_lock.insert(received_user_object.get_user_data().clone(), received_user_object.get_offer_data().clone());

        println!("Offer type: {:?}", received_user_object.get_post_type());
    }
}


// the get request is used to retrieve data from the server -> we will retrieve the best possible offer based on the user's data (what their budget is)
fn handle_get_request(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    println!("GET request");
}

fn match_orders(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    let mut read_lock = offers_log.read().unwrap();
    let sorted_offers = read_lock.clone();

    // sort the offers by price

    // sorted_offers.sort_by(|a, b| a.get_price().cmp(&b.get_price()));
    // println!("Sorted offers: {:?}", sorted_offers);
}


fn handle_serialised_user_object(mut stream: TcpStream, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {

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
                    handle_post_request(received_user_object, user_log.clone(), offers_log.clone());
                }

                else {
                    // println!("GET request");
                    handle_get_request(received_user_object, user_log.clone(), offers_log.clone());
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
    // creating that global variables allowing for concurrent access (writes and reads)

    // list of logged in users
    let user_log:Arc<RwLock<HashMap<User, bool>>> = Arc::new(RwLock::new(HashMap::new()));

    // list of offers
    let offers_log: Arc<RwLock<HashMap<User, Offer>>> = Arc::new(RwLock::new(HashMap::new()));


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and spawn a thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{:?}", stream.local_addr().unwrap());
                println!("handlam");

                let user_log_clone = user_log.clone();
                let offers_log_clone = offers_log.clone();
                std::thread::spawn(|| {handle_serialised_user_object(stream, user_log_clone, offers_log_clone)});

            }
            Err(e) => {
                eprintln!("Error accepting connection: {}\n", e);
            }
        }
    }
}