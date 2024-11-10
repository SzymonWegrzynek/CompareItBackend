use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
    pub email: String,
    pub password: String,
}
