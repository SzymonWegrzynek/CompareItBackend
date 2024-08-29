use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct InsertImageRequest {
    pub file_path: String,
    pub phone_id: i32
}
