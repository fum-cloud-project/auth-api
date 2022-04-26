use actix::MessageResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
pub struct Resources {
    pub method: Method,
    pub path: String,
    pub access: u16,
}
