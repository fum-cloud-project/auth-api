use actix::MessageResponse;
use bson::Bson;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, MessageResponse)]
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
    INVALID,
}

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
pub struct Resources {
    pub method: Method,
    pub path: String,
    pub access: i32,
}

impl Resources {
    pub fn new(
        create_resource: &crate::actors::database::resources::CreateOrUpdateResource,
    ) -> Resources {
        Resources {
            method: create_resource.method,
            path: create_resource.path.clone(),
            access: create_resource.access,
        }
    }
}

impl std::convert::From<Method> for Bson {
    fn from(method: Method) -> Self {
        Bson::String(match method {
            Method::CONNECT => format!("CONNECT"),
            Method::DELETE => format!("DELETE"),
            Method::GET => format!("GET"),
            Method::HEAD => format!("HEAD"),
            Method::OPTIONS => format!("OPTIONS"),
            Method::PATCH => format!("PATCH"),
            Method::POST => format!("POST"),
            Method::PUT => format!("PUT"),
            Method::TRACE => format!("TRACE"),
            Method::INVALID => format!("INVALID"),
        })
    }
}
