// implementing the Request struct


// mod user;
use crate::User;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Request {
    pub request_type: String,
    pub post_type: String,
    pub req_type: String,
    pub user_data: User,

}

#[allow(dead_code)]
impl Request {
    pub fn new(request_type: String, post_type: String, req_type: String, user_data: User) -> Request {
        Request {request_type, post_type, req_type, user_data}
    }
    pub fn get_request_type(&self) -> &String {
        &self.request_type
    }
    pub fn get_user_data(&self) -> &User {
        &self.user_data
    }
}