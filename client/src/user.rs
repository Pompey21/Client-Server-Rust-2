
use std::collections::HashMap;

// implementing the User struct
#[derive(Clone)]
pub struct User {
    user_name: String,
    password: String,
    funds: u32,
    memory: u32,
    offer: bool,
}

impl User {
    pub fn new(user_name: String, password:String, funds: u32, memory:u32, offer:bool) -> User {
        User {user_name, password,funds, memory,offer}
    }   
    fn get_user_name(&self) -> &String {
        &self.user_name
    }
    fn get_password(&self) -> &String {
        &self.password
    }
    fn get_funds(&self) -> &u32 {
        &self.funds
    }
    fn get_memory(&self) -> &u32 {
        &self.memory
    }
    fn get_offer(&self) -> &bool {
        &self.offer
    }
    fn set_offer(&mut self, offer: bool) {
        self.offer = offer;
    }
    fn set_funds(&mut self, funds: u32) {
        self.funds = funds;
    }
    fn set_memory(&mut self, memory: u32) {
        self.memory = memory;
    }
}