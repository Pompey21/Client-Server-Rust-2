// implementing the Offer struct

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Offer {
    username: String,
    offer_time: u32,
    offer_space: u32,
    offer_price: u32,
}

#[allow(dead_code)]
impl Offer {
    pub fn new(username: String, offer_time: u32, offer_space: u32, offer_price: u32) -> Offer {
        Offer {username, offer_time, offer_space, offer_price}
    }
    pub fn get_username(&self) -> &String {
        &self.username
    }
    pub fn get_offer_time(&self) -> &u32 {
        &self.offer_time
    }
    pub fn get_offer_space(&self) -> &u32 {
        &self.offer_space
    }
    pub fn get_offer_price(&self) -> &u32 {
        &self.offer_price
    }
}