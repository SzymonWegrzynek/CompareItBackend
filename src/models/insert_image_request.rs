use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct InsertImageRequest {
    pub phone_id: i32,
    pub file_path: String
}
