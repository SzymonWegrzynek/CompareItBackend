use crate::{
    models::phone::{PhoneFullSpec, PhoneFullSpecResponse},
    modules::stock_image::StockImage,
};

pub struct ChangePhoneResponse;

impl ChangePhoneResponse {
    pub fn change_phone_response(phone: PhoneFullSpec) -> PhoneFullSpecResponse {
        let images_base64: Option<Vec<String>> = phone.images.as_ref().map(|image_data| {
            image_data
                .iter()
                .map(|img| StockImage { data: img.clone() }.to_base64())
                .collect()
        });

        PhoneFullSpecResponse {
            phone_id: phone.phone_id,
            producer: phone.producer,
            model: phone.model,
            price: phone.price,
            battery_capacity: phone.battery_capacity,
            screen_size: phone.screen_size,
            screen_technology: phone.screen_technology,
            resolution: phone.resolution,
            screen_refresh_rate: phone.screen_refresh_rate,
            ram_memory: phone.ram_memory,
            front_camera: phone.front_camera,
            rear_camera: phone.rear_camera,
            processor_name: phone.processor_name,
            processor_cores: phone.processor_cores,
            operating_system: phone.operating_system,
            water_dust_resistance: phone.water_dust_resistance,
            colors: phone.colors,
            sim_cards: phone.sim_cards,
            memory_variants: phone.memory_variants,
            connectivities: phone.connectivities,
            images: images_base64,
        }
    }
}
