// implementing the struct for sending a POST reqeust for a user

use crate::User;
use crate::Offer;

// ====================================================================================================
// Implementing the POST structs
// ====================================================================================================

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct POST_User {
    user: User,
}

#[allow(dead_code)]
impl POST_User {
    pub fn new(user: User) -> POST_User {
        POST_User {user}
    }
    pub fn get_user(&self) -> &User {
        &self.user
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct POST_Offer {
    offer: Offer,
}

#[allow(dead_code)]
impl POST_Offer {
    pub fn new(offer: Offer) -> POST_Offer {
        POST_Offer {offer}
    }
    pub fn get_offer(&self) -> &Offer {
        &self.offer
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum Either {
    POST_User(POST_User),
    POST_Offer(POST_Offer),
}


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct POST_Request {
    request_type: String,
    data_load: Either,
}

#[allow(dead_code)]
impl POST_Request {
    pub fn new(request_type: String, data_load: Either) -> POST_Request {
        POST_Request {request_type, data_load}
    }
    pub fn get_request_type(&self) -> &String {
        &self.request_type
    }
    pub fn get_data_load(&self) -> &Either {
        &self.data_load
    }
}

// ====================================================================================================
// ====================================================================================================
// Implementing GET structs
// ====================================================================================================

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct GET_Offer {
    memory: u32,
    time: u32,
    deposit: u32,
}

#[allow(dead_code)]
impl GET_Offer {
    pub fn new(memory: u32, time: u32, deposit: u32) -> GET_Offer {
        GET_Offer {memory, time, deposit}
    }
    pub fn get_memory(&self) -> &u32 {
        &self.memory
    }
    pub fn get_time(&self) -> &u32 {
        &self.time
    }
    pub fn get_deposit(&self) -> &u32 {
        &self.deposit
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct GET_Request {
    request_type: String,
    data_load: GET_Offer,
}

#[allow(dead_code)]
impl GET_Request {
    pub fn new(request_type: String, data_load: GET_Offer) -> GET_Request {
        GET_Request {request_type, data_load}
    }
    pub fn get_request_type(&self) -> &String {
        &self.request_type
    }
    pub fn get_data_load(&self) -> &GET_Offer {
        &self.data_load
    }
}


// ====================================================================================================
// ====================================================================================================
// Implementing the request struct
// ====================================================================================================
// ====================================================================================================
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Request_1<T> {
    request_type: String,
    data_load: T,
}

#[allow(dead_code)]
impl Request_1<POST_Request> {
    pub fn new(request_type: String, data_load: POST_Request) -> Request_1<POST_Request> {
        Request_1 {request_type, data_load}
    }
    pub fn get_request_type(&self) -> &String {
        &self.request_type
    }
    pub fn get_data_load(&self) -> &POST_Request {
        &self.data_load
    }
}

#[allow(dead_code)]
impl Request_1<GET_Request> {
    pub fn new(request_type: String, data_load: GET_Request) -> Request_1<GET_Request> {
        Request_1 {request_type, data_load}
    }
    pub fn get_request_type(&self) -> &String {
        &self.request_type
    }
    pub fn get_data_load(&self) -> &GET_Request {
        &self.data_load
    }
}
