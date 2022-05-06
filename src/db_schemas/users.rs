use actix::MessageResponse;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
pub struct PasswordlessUsers {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub access_level: i32,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, MessageResponse)]
pub struct Users {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub access_level: i32,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub is_deleted: bool,
}

impl Users {
    pub fn new(create_user: crate::actors::database::users::CreateUser) -> Users {
        Users {
            _id: ObjectId::new(),
            first_name: create_user.first_name,
            last_name: create_user.last_name,
            email: create_user.email,
            password: create_user.password,
            access_level: create_user.access_level,
            is_deleted: false,
            creation_date: chrono::Utc::now(),
        }
    }
}
