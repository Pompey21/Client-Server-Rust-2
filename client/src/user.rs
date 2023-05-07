// implementing the User struct
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    user_name: String,
    password: String,
    funds: u32,
    memory: u32,
    offer: bool,
}

#[allow(dead_code)]
impl User {
    pub fn new(user_name: String, password:String, funds: u32, memory:u32, offer:bool) -> User {
        User {user_name, password,funds, memory,offer}
    }   
    pub fn get_user_name(&self) -> &String {
        &self.user_name
    }
    pub fn get_password(&self) -> &String {
        &self.password
    }
    pub fn get_funds(&self) -> &u32 {
        &self.funds
    }
    pub fn get_memory(&self) -> &u32 {
        &self.memory
    }
    pub fn get_offer(&self) -> &bool {
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