use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDataSignIn {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataSignOut {
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataCreate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub access_level: Option<i32>,
}

pub type UserDataUpdateAdmin = UserDataCreate;

#[derive(Serialize, Deserialize)]
pub struct UserDataUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataRefresh {
    pub refresh_token: Option<String>,
}
