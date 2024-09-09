use actix_web::{web, HttpResponse};

use crate::state::AppState;
use crate::models::images::Image;
use crate::models::insert_image_request::InsertImageRequest;
use crate::modules::insert_image::StockImage;


pub struct ImageController;


impl ImageController {
    pub async fn get_all_images(app_state: web::Data<AppState>) -> HttpResponse {
        match sqlx::query_as!(Image, "SELECT * FROM Images")
        .fetch_all(&app_state.pool)
        .await 
        {
            Ok(images) => HttpResponse::Ok().json(images),
            Err(_) => HttpResponse::BadRequest().into()
        }
    }


    pub async fn insert_image(app_state: web::Data<AppState>, form: web::Json<InsertImageRequest>) -> HttpResponse {
        let stock_image = StockImage::get_stock_image(&form.file_path);

        match sqlx::query!("INSERT INTO Images (phone_id, image) VALUES ($1, $2)", &form.phone_id, stock_image.data)
        .execute(&app_state.pool)
        .await 
        {
            Ok(_) => HttpResponse::Created().into(),
            Err(_) => HttpResponse::BadRequest().into()
        }
    }


    pub async fn get_image(app_state: web::Data<AppState>, path: web::Path<usize>) -> HttpResponse {
        let phone_id: usize = path.into_inner();

        let result: sqlx::Result<Vec<Image>> = sqlx::query_as!(
            Image, 
            "SELECT id, phone_id, image FROM Images WHERE phone_id = $1", 
            phone_id as i64,
        ).fetch_all(&app_state.pool).await;

        match result {
            Ok(images) => {
                let image_to_base64: Vec<String> = images.into_iter()
                .map(|image| {
                    let stock_image = StockImage {data: image.image }; 
                    stock_image.to_base64()
                })
                .collect(); 
                HttpResponse::Ok().json(image_to_base64)
            },
            _ => HttpResponse::NotFound().into()
        }
    }
}
