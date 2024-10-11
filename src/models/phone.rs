use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Phone {
    pub phone_id: i32,
    pub producer: String,
    pub model: String,
    pub price: i32,
    pub battery_capacity: i32,
    pub screen_size: String,
    pub screen_technology: String,
    pub resolution: String,
    pub screen_refresh_rate: i32,
    pub ram_memory: i32,
    pub front_camera: String,
    pub rear_camera: String,
    pub processor_name: String,
    pub processor_cores: i32,
    pub operating_system: String,
    pub water_dust_resistance: String,
    pub colors: Option<Vec<String>>,
    pub sim_cards: Option<Vec<String>>,
    pub memory_variants: Option<Vec<i32>>,
    pub connectivities: Option<Vec<String>>,
    pub images: Option<Vec<Vec<u8>>>,
}
