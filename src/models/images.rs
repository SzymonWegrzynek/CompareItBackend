use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub phone_id: Option<i32>,
    pub image: Vec<u8>,
}
