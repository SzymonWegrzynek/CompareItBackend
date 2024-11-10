use actix_web::{web, HttpResponse};

use crate::models::image::InsertImageRequest;
use crate::modules::stock_image::StockImage;
use crate::state::AppState;

pub struct ImageHandler;

impl ImageHandler {
    pub async fn insert_image(
        app_state: web::Data<AppState>,
        form: web::Json<InsertImageRequest>,
    ) -> HttpResponse {
        let stock_image = StockImage::get_stock_image(&form.image_url);

        match sqlx::query_file!(
            "src/queries/insert_image.sql",
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
}
