use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub image_id: i32,
    pub model_id: Option<i32>,
    pub image_url: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct InsertImageRequest {
    pub model_id: i32,
    pub image_url: String,
}
