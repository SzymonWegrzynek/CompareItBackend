use actix_web::{web, HttpResponse};

use crate::models::phone::{PhoneForSearch, PhoneFullSpec, PhoneFullSpecResponse};
use crate::modules::insert_image::StockImage;
use crate::state::AppState;

pub struct PhoneController;

impl PhoneController {
    pub async fn health_check() -> HttpResponse {
        HttpResponse::Ok().into()
    }

    pub async fn get_phone(app_state: web::Data<AppState>, path: web::Path<usize>) -> HttpResponse {
        let phone_id: usize = path.into_inner();

        let phone: sqlx::Result<PhoneFullSpec> = sqlx::query_file_as!(
            PhoneFullSpec,
            "src/queries/get_phone_by_id.sql",
            phone_id as i64,
        )
        .fetch_one(&app_state.pool)
        .await;

        match phone {
            Ok(phone) => {
                let images_base64: Option<Vec<String>> = phone.images.as_ref().map(|image_data| {
                    image_data
                        .iter()
                        .map(|img| StockImage { data: img.clone() }.to_base64())
                        .collect()
                });

                let response = PhoneFullSpecResponse {
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
                };

                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::NotFound().into(),
        }
    }

    pub async fn get_all_phones(app_state: web::Data<AppState>) -> HttpResponse {
        match sqlx::query_file_as!(PhoneForSearch, "src/queries/get_phone.sql")
            .fetch_all(&app_state.pool)
            .await
        {
            Ok(phones) => HttpResponse::Ok().json(phones),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }
}
