use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Image {
    pub images: Option<Vec<u8>>
}
