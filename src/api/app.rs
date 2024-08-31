use sqlx::postgres::PgQueryResult; 
use actix_web::{web, HttpResponse};

use crate::state::AppState;
use crate::models::phones::Phone;
use crate::models::images::Image;
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
        "SELECT * FROM Phones WHERE id = $1",
        phone_id as i64,
    ).fetch_one(&app_state.pool).await;

    match phone {
        Ok(phone) => HttpResponse::Ok().json(phone),
        Err(_) => HttpResponse::BadRequest().into()
    }
}


pub async fn get_all_phones(app_state: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(Phone, "SELECT * FROM Phones").fetch_all(&app_state.pool).await {
        Ok(phones) => HttpResponse::Ok().json(phones),
        Err(_) => HttpResponse::BadRequest().into()
    }
}


pub async fn delete_phone(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let phone_id: usize = path.into_inner();

    let deleted: sqlx::Result<PgQueryResult> = sqlx::query!(
        "DELETE FROM Phones WHERE id = $1",
        phone_id as i64,
    ).execute(&app_state.pool).await;

    match deleted {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}


pub async fn get_all_images(app_state: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(Image, "SELECT * FROM Images").fetch_all(&app_state.pool).await {
        Ok(images) => HttpResponse::Ok().json(images),
        Err(_) => HttpResponse::BadRequest().into()
    }
}


pub async fn insert_image(app_state: web::Data<AppState>, form: web::Json<InsertImageRequest>) -> HttpResponse {
    let image = get_stock_image(&form.file_path);

    match sqlx::query!("INSERT INTO Images (phone_id, image) VALUES ($1, $2)", &form.phone_id, &image).execute(&app_state.pool).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into()
    }
}


pub async fn get_image(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let phone_id: usize = path.into_inner();

    let result: sqlx::Result<Image> = sqlx::query_as!(Image, "SELECT id, phone_id, image FROM Images WHERE phone_id = $1", phone_id as i64).fetch_one(&app_state.pool).await;

    match result {
        Ok(image) => {let image_to_base64 = to_base64(image.image); HttpResponse::Ok().json(image_to_base64)},
        Err(_) => HttpResponse::BadRequest().into()
    }
}
