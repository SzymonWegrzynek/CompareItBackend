use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;

mod api;
mod database;
mod modules;
mod state;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    dotenv().ok();

    let pool = database::conn::create_pool().await;

    let app_state = state::AppState {pool};

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(app_state.clone()))
        .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
        .route("/health-check", web::get().to(api::app::health_check))
        .route("/get/{phone_id}", web::get().to(api::app::get_phone))
        .route("/delete/{phone_id}", web::delete().to(api::app::delete_phone))
        .route("/get-all", web::get().to(api::app::get_all))
        .route("/insert-image", web::post().to(api::app::insert_image))
    }).bind(("localhost", 8000))?
    .run()
    .await?;

    Ok(())
}
