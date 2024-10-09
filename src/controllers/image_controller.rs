use actix_web::{web, HttpResponse};

use crate::models::image::Image;
use crate::models::insert_image_request::InsertImageRequest;
use crate::modules::insert_image::StockImage;
use crate::state::AppState;

pub struct ImageController;

impl ImageController {
    pub async fn get_all_images(app_state: web::Data<AppState>) -> HttpResponse {
        match sqlx::query_as!(Image, "SELECT * FROM Image")
            .fetch_all(&app_state.pool)
            .await
        {
            Ok(images) => HttpResponse::Ok().json(images),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }

    pub async fn insert_image(
        app_state: web::Data<AppState>,
        form: web::Json<InsertImageRequest>,
    ) -> HttpResponse {
        let stock_image = StockImage::get_stock_image(&form.image_url);

        match sqlx::query!(
            "INSERT INTO Image (model_id, image_url) VALUES ($1, $2)",
            &form.model_id,
            stock_image.data
        )
        .execute(&app_state.pool)
        .await
        {
            Ok(_) => HttpResponse::Created().into(),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }

    pub async fn get_image(app_state: web::Data<AppState>, path: web::Path<usize>) -> HttpResponse {
        let phone_id: usize = path.into_inner();

        let result: sqlx::Result<Vec<Image>> = sqlx::query_as!(
            Image,
            "SELECT image_id, model_id, image_url FROM Image WHERE model_id = $1",
            phone_id as i64,
        )
        .fetch_all(&app_state.pool)
        .await;

        match result {
            Ok(images) => {
                let image_to_base64: Vec<String> = images
                    .into_iter()
                    .map(|image| {
                        let stock_image = StockImage {
                            data: image.image_url,
                        };
                        stock_image.to_base64()
                    })
                    .collect();
                HttpResponse::Ok().json(image_to_base64)
            }
            _ => HttpResponse::NotFound().into(),
        }
    }
}
