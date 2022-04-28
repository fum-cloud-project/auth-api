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
pub struct UserDataSignUp {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataRefresh {
    pub refresh_token: Option<String>,
}
