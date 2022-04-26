use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub access_level: u16,
    pub creation_date: chrono::NaiveDateTime,
    pub is_deleted: bool,
}
