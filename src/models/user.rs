use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInUserData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub message: String,
    pub token: String,
}
