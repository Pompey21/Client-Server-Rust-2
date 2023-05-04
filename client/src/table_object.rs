use crate::user::User;
use std::collections::HashMap;

// implementing the TableObject struct
#[derive(Clone)]
pub struct Table {
    user_name: String,
    password: String,
    funds: u32,
    memory: u32,
    offer: bool,
}


// impl User {
//     fn new(user_name: String, password:String, funds: u32, memory:u32, offer:bool) -> User {
//         User {user_name, password,funds, memory,offer}
//     }

//     fn get_user_offer(userX: &mut User, map: &mut HashMap<String,u32>) -> (String, u32) {
//         if map.contains_key(&userX.user_name) {
//             return (userX.user_name.clone() ,userX.memory);
//         }
//         return ("".to_string(),0);
//     }

//     fn insert_offer(userX: &mut User, map: &mut HashMap<String, u32>){
//         if map.contains_key(&userX.user_name) {
//             println!("This user has already submitted an offer!");
//         }
//         else {
//             map.insert(userX.user_name.clone(), userX.memory);
//         }
//     }
// }



