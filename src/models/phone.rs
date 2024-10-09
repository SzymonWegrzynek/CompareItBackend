use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Phone {
    pub phone_id: i32,
    pub producer_id: i32,
    pub model_id: i32,
    pub color_id: i32,
    pub internal_memory_id: i32,
    pub battery_id: i32,
    pub screen_id: i32,
    pub ram_id: i32,
    pub camera_id: i32,
    pub processor_id: i32,
    pub operating_system_id: i32,
    pub connectivity_id: i32,
    pub water_dust_resistance_id: i32,
    pub sim_card_id: i32,
    pub image_id: Option<i32>,
}
