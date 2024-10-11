use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod controllers;
mod database;
mod models;
mod modules;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_conn = database::conn::DatabaseConn::create_pool().await;
    let pool = db_conn.pool().clone();

    let app_state = state::AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route(
                "/healthcheck",
                web::get().to(controllers::phone_controller::PhoneController::health_check),
            )
            .route(
                "/get-phone/{phone_id}",
                web::get().to(controllers::phone_controller::PhoneController::get_phone),
            )
            .route(
                "/get-all-phones",
                web::get().to(controllers::phone_controller::PhoneController::get_all_phones),
            )
            .route(
                "/get-all-images",
                web::get().to(controllers::image_controller::ImageController::get_all_images),
            )
            .route(
                "/insert-image",
                web::post().to(controllers::image_controller::ImageController::insert_image),
            )
            .route(
                "/get-image/{phone_id}",
                web::get().to(controllers::image_controller::ImageController::get_image),
            )
    })
    .bind(("localhost", 8000))?
    .run()
    .await?;

    Ok(())
}
