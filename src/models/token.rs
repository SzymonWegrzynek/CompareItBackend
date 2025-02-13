use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerifyToken {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProtectRoute {
    pub id: String,
}
