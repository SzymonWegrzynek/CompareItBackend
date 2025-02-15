use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerifyToken {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProtectRoute {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProtectRouteResponse {
    pub role: String,
}
