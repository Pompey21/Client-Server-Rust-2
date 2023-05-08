// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream}, collections::HashMap, sync::{Arc, RwLock}, fs::File, io::BufReader};

mod requests;
mod request;
mod user;
mod offer;

use requests::{POST_User, POST_Offer, POST_Request, GET_Offer, GET_Request, Request, Either, Either_Request, Request_1};

use user::User;
use offer::Offer;
use std::io::BufRead;


#[tokio::main]
async fn main() {
    // creating that global variables allowing for concurrent access (writes and reads)

    // list of logged in users
    let user_log:Arc<RwLock<HashMap<User, bool>>> = Arc::new(RwLock::new(HashMap::new()));

    // list of offers
    let offers_log: Arc<RwLock<HashMap<User, Offer>>> = Arc::new(RwLock::new(HashMap::new()));

    // initialising the offers
    let parsed_data: Vec<(User,Offer)> = read_from_txt_file("/Users/admin/Desktop/Client-Server-Rust-2/server/src/offers_init.txt".to_string());
    initialise_global_variables(parsed_data, user_log.clone(), offers_log.clone());


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and spawn a thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{:?}", stream.local_addr().unwrap());

                let user_log_clone = user_log.clone();
                let offers_log_clone = offers_log.clone();
                std::thread::spawn(|| {receive_message(stream, user_log_clone, offers_log_clone)});

            }
            Err(e) => {
                eprintln!("Error accepting connection: {}\n", e);
            }
        }
    }
}

// ===========================================================
// ===========================================================
//  Initialising the offers
// ===========================================================
// Reading the data from the text file
fn read_from_txt_file(text_file_name: String) -> Vec<(User, Offer)> {
    let mut parsed_data: Vec<(User, Offer)> = Vec::new();
    let file = File::open(text_file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        let line_vector: Vec<&str> = line_string.split(";").collect();
        // println!("{:?}", line_vector);
        let (user_new, offer_new) = generate_instances(line_vector);
        // println!("User: {:?}, Offer: {:?}", user_new, offer_new);
        parsed_data.push((user_new, offer_new));
    }
    return parsed_data;
}

// Parsing the data from the text file
fn generate_instances(line_vector: Vec<&str>) -> (User, Offer) {
    let user_new: User = User::new(line_vector[0].to_string(), line_vector[1].to_string(), line_vector[2].parse::<u32>().unwrap(), line_vector[3].parse::<u32>().unwrap(), true);
    let offer_new: Offer = Offer::new(100, line_vector[3].parse::<u32>().unwrap(), line_vector[4].parse::<u32>().unwrap());
    return (user_new, offer_new);
}

// Initialising the global variables
fn initialise_global_variables(parsed_data: Vec<(User,Offer)>, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    let mut write_lock_offers = offers_log.write().unwrap();
    let mut write_lock_users = user_log.write().unwrap();
    for (user, offer) in parsed_data {
        write_lock_offers.insert(user.clone(), offer.clone());
        write_lock_users.insert(user.clone(), true);
    }
}
// ===========================================================
// ===========================================================
// Handle the requests
// ===========================================================
// ===========================================================
// receive the message
fn receive_message(mut stream: TcpStream, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    
    // Read the message length from the header
    let mut header = [0; HEADER_SIZE];
    stream.read_exact(&mut header).unwrap();
    let message_len = u32::from_be_bytes(header);
    println!("Message length: {}", message_len);

    // Allocate a buffer to receive the message payload
    let mut buffer = vec![0; message_len as usize];

    // Read the message payload into the buffer
    // stream.read_exact(&mut buffer).unwrap();


    loop {
        match stream.read(&mut buffer) {
            // println!("Received: {:?}\n", buffer);
            Ok(n) if n == 0 => {
                // connection closed
                break;
            }
            Ok(n) => {
                println!("n: {}", n);

                // send response to client
                stream.write_all("200 OK".to_string().as_bytes()).unwrap();
                println!("Sent: 200 OK\n");

                // handle the received message
                // let received_user_object: Request_1<T> = bincode::deserialize::<Request_1<T>>(&buffer).unwrap();
                // println!("Received: {:?}\n", received_user_object);

            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                stream.write_all("400 BAD REQUEST".to_string().as_bytes()).unwrap();
                // break;
            }
        }
    }
}


// Handling the serialised user object
const HEADER_SIZE: usize = 4;
fn handle_serialised_user_object(mut stream: TcpStream, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    
    // Read the message length from the header
    let mut header = [0; HEADER_SIZE];
    stream.read_exact(&mut header).unwrap();
    let message_len = u32::from_be_bytes(header);
    println!("Message length: {}", message_len);

    // Allocate a buffer to receive the message payload
    let mut buffer = vec![0; message_len as usize];

    // Read the message payload into the buffer
    // stream.read_exact(&mut buffer).unwrap();


    loop {
        match stream.read(&mut buffer) {
            // println!("Received: {:?}\n", buffer);
            Ok(n) if n == 0 => {
                // connection closed
                break;
            }
            Ok(n) => {
                println!("n: {}", n);

                // send response to client
                stream.write_all("200 OK".to_string().as_bytes()).unwrap();
                println!("Sent: 200 OK\n");

                // handle the received message
                // let received_user_object: Request_1<T> = bincode::deserialize::<Request_1<T>>(&buffer).unwrap();
                // println!("Received: {:?}\n", received_user_object);

            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                stream.write_all("400 BAD REQUEST".to_string().as_bytes()).unwrap();
                // break;
            }
        }
    }
}






// Handling the POST requests
fn handle_post_request(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {

    println!("Received: {:?}\n", received_user_object);

    

    // if received_user_object.get_post_type().starts_with("USER") {
    //     let mut write_lock = user_log.write().unwrap();
    //     write_lock.insert(received_user_object.get_user_data().clone(), true);
    //     println!("User logged in: {:?}", write_lock);
    // }

    // if received_user_object.get_post_type().starts_with("OFFER") {
    //     let mut write_lock = offers_log.write().unwrap();
    //     write_lock.insert(received_user_object.get_user_data().clone(), received_user_object.get_offer_data().clone());

    //     println!("Offer type: {:?}", received_user_object.get_post_type());
    // }
}

// Handling the GET requests
// the get request is used to retrieve data from the server -> we will retrieve the best possible offer based on the user's data (what their budget is)
fn handle_get_request(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    println!("GET request");
}

fn match_orders(received_user_object: Request, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) -> Vec<(Offer,User)> {

    let mut read_lock = offers_log.read().unwrap();
    let sorted_offers = read_lock.clone();

    let mut sorted_offers_vec: Vec<(Offer,User)> = Vec::new();

    for (user, offer) in sorted_offers {
        sorted_offers_vec.push((offer, user));
    }

    // filter the offers


    // sort the offers by price
    sorted_offers_vec.sort_by(|a, b| a.0.get_offer_price().cmp(&b.0.get_offer_price()));
    println!("Sorted offers: {:?}", sorted_offers_vec);

    return sorted_offers_vec;

}

