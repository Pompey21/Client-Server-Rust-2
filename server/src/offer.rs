// implementing the Offer struct

use crate::user::User;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Offer {
    offer_time: u32,
    offer_space: u32,
    offer_price: u32,
}

#[allow(dead_code)]
impl Offer {
    pub fn new(offer_time: u32, offer_space: u32, offer_price: u32) -> Offer {
        Offer {offer_time, offer_space, offer_price}
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