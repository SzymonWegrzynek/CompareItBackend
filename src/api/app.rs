use sqlx::postgres::PgQueryResult; 
use actix_web::{web, HttpResponse};

use crate::state::AppState;
use crate::models::phone::Phone;
use crate::models::image::Image;
use crate::models::insert_image_request::InsertImageRequest;
use crate::modules::insert_image::get_stock_image;
use crate::modules::insert_image::to_base64;


pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().into()
}


pub async fn get_phone(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let phone_id: usize = path.into_inner();

    let phone: sqlx::Result<Phone> = sqlx::query_as!(
        Phone,
        "SELECT * FROM phones WHERE id = $1",
        phone_id as i64,
    ).fetch_one(&app_state.pool).await;

    match phone {
        Ok(phone) => HttpResponse::Ok().json(phone),
        Err(_) => HttpResponse::BadRequest().into()
    }
}


pub async fn delete_phone(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let phone_id: usize = path.into_inner();

    let deleted: sqlx::Result<PgQueryResult> = sqlx::query!(
        "DELETE FROM phones WHERE id = $1",
        phone_id as i64,
    ).execute(&app_state.pool).await;

    match deleted {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}


pub async fn get_all(app_state: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(
        Phone,
        "SELECT * FROM phones"
    ).fetch_all(&app_state.pool).await {
        Ok(phones) => HttpResponse::Ok().json(phones),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


pub async fn insert_image(app_state: web::Data<AppState>, form: web::Json<InsertImageRequest>) -> HttpResponse {
    let image = get_stock_image(&form.file_path);

    match sqlx::query!("UPDATE phones SET images = $1 WHERE id = $2", image, &form.phone_id).execute(&app_state.pool).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into()
    }
}

pub async fn get_image(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let phone_id: usize = path.into_inner();

    let result: sqlx::Result<Image> = sqlx::query_as!(Image, "SELECT images FROM phones WHERE id = $1", phone_id as i64).fetch_one(&app_state.pool).await;

    match result {
        Ok(image) => {let image_to_base64 = to_base64(image.images); HttpResponse::Ok().json(image_to_base64)},
        Err(_) => HttpResponse::BadRequest().into()
    }
}
