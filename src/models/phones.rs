use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Phone {
    pub id: i32,
    pub manufacturer: String,
    pub model: String,
    pub price: String,
    pub colors: Option<String>,
    pub internal_memory_variants: Option<String>,
    pub battery_capacity: Option<String>,
    pub screen_size: Option<String>,
    pub screen_technology: Option<String>,
    pub resolution: Option<String>,
    pub screen_refresh_rate: Option<String>,
    pub ram_memory: Option<String>,
    pub camera: Option<String>,
    pub processor: Option<String>,
    pub processor_speed: Option<String>,
    pub processor_cores: Option<i32>,
    pub operating_system: Option<String>,
    pub connectivity: Option<String>,
    pub dust_water_resistance_standard: Option<String>,
    pub sim_card_type: Option<String>,
}
