use actix_web::{web, HttpResponse};

use crate::{models::image::InsertImageRequest, modules::stock_image::StockImage, state::AppState};

pub struct ImageHandler;

impl ImageHandler {
    pub async fn insert_image(
        app_state: web::Data<AppState>,
        payload: web::Json<InsertImageRequest>,
    ) -> HttpResponse {
        let stock_image = StockImage::get_stock_image(&payload.image_url);

        match sqlx::query_file!(
            "src/queries/insert_image.sql",
            &payload.model_id,
            stock_image.data
        )
        .execute(&app_state.pool)
        .await
        {
            Ok(_) => HttpResponse::Created().into(),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }
}
