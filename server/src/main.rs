// ======================================
// SERVER IMPLEMENTATION
// ======================================

#[allow(dead_code)]
use std::io::{Read, Write};
use std::{net::{TcpListener, TcpStream}, collections::HashMap, sync::{Arc, RwLock}, fs::File, io::BufReader};

mod requests;
mod user;
mod offer;

use requests::{POST_User, POST_Offer, POST_Request, GET_Offer, GET_Request, Either, Request_1};

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
const HEADER_SIZE: usize = 4;
fn receive_message(mut stream: TcpStream, user_log: Arc<RwLock<HashMap<User, bool>>>, offers_log: Arc<RwLock<HashMap<User, Offer>>>) {
    
    // Read the message length from the header
    let mut header = [0; HEADER_SIZE];
    stream.read_exact(&mut header).unwrap();
    let message_len = u32::from_be_bytes(header);
    println!("Message length: {}", message_len);

    // Allocate a buffer to receive the message payload
    let mut buffer = vec![0; message_len as usize];
    println!("Buffer type: {}", std::any::type_name::<Vec<u8>>());

    // Read the message payload into the buffer
    // stream.read_exact(&mut buffer).unwrap();


    loop {
        match stream.read(&mut buffer) {
            // println!("Received: {:?}\n", buffer);
            Ok(n) if n == 0 => {
                // connection closed
                break;
            }
            // Handling POST_User
            Ok(n) if n == 61 => {
                // send response to client
                stream.write_all("200 OK".to_string().as_bytes()).unwrap();
                println!("Sent: 200 OK\n");

                // handle the received message
                let received_user_object = deserialise_post_req(buffer.clone());

                match received_user_object.get_data_load().get_data_load() {
                    Either::POST_User(user) => {
                        let rec_usr_obj = user.get_user();
                        println!("Received user object: {:?}", rec_usr_obj);

                        // add the user to the user_log
                        let mut write_lock = user_log.write().unwrap();
                        write_lock.insert(rec_usr_obj.clone(), true);

                        // // add the user to the offers_log - just as a key
                        // let mut write_lock = offers_log.write().unwrap();
                        // write_lock.insert(rec_usr_obj.clone(), Offer::new(0, 0, 0));
                    },
                    Either::POST_Offer(offer) => {}
                }
                println!("Received: {:?}\n", received_user_object);

            } 
            // Handling POST_Offer
            Ok(n) if n == 40 => {
                // send response to client
                stream.write_all("200 OK".to_string().as_bytes()).unwrap();
                println!("Sent: 200 OK\n");

                // handle the received message
                let received_user_object = deserialise_post_req(buffer.clone());


                match received_user_object.get_data_load().get_data_load() {
                    Either::POST_User(user) => {},
                    Either::POST_Offer(offer) => {
                        let rec_ofr_obj = offer.get_offer();
                        println!("Received offer object: {:?}", rec_ofr_obj);

                        // // add the offer to the offers_log
                        // let mut write_lock = offers_log.write().unwrap();
                        // write_lock.insert(rec_ofr_obj.clone(), offer.clone());
                    }
                }
                println!("Received: {:?}\n", received_user_object);
            }
            // Handling GET_Offer
            Ok(n) if n == 34 => {
                // send response to client
                stream.write_all("200 OK".to_string().as_bytes()).unwrap();
                println!("Sent: 200 OK\n");

                // handle received message
                let received_user_object: Request_1<GET_Request> = deserialise_get_req(buffer.clone());
                println!("Received: {:?}", received_user_object);
                // unpack the message
                let received_offer: GET_Offer = unpack_get_req(received_user_object);
                println!("Received user: {:?}", received_offer);
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




// ===========================================================
// Deserialise POST and GET requests
// ===========================================================
// Deserialise post request
fn deserialise_post_req(mut buffer: Vec<u8>) -> Request_1<POST_Request> {
    let received_user_object: Request_1<POST_Request> = bincode::deserialize::<Request_1<POST_Request>>(&buffer).unwrap();
    // println!("Received: {:?}\n", received_user_object);
    return received_user_object;
}

// Deserialise get request
fn deserialise_get_req(mut buffer: Vec<u8>) -> Request_1<GET_Request> {
    let received_user_object: Request_1<GET_Request> = bincode::deserialize::<Request_1<GET_Request>>(&buffer).unwrap();
    // println!("Received: {:?}\n", received_user_object);
    return received_user_object;
}
// ===========================================================
// ===========================================================
// Handle deserialised objects -> unpack the data
// ===========================================================
fn unpack_get_req(received_user_object: Request_1<GET_Request>) -> GET_Offer {
    let received_user = received_user_object.get_data_load().get_data_load();
    return received_user.clone();
}







