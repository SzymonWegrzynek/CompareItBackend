use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InsertImageRequest {
    pub model_id: i32,
    pub image_url: String,
}
