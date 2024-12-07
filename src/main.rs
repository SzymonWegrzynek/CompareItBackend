use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod database;
mod extractors;
mod handlers;
mod models;
mod modules;
mod routes;
mod state;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_conn = database::conn::DatabaseConn::create_pool().await;
    let pool = db_conn.pool().clone();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = state::AppState { pool, jwt_secret };

    let server_ip = env::var("SERVER_IP").expect("SERVER_IP must be set");
    let server_port = env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set")
        .parse::<u16>()
        .expect("Invalid port number");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(routes::routes::healthcheck)
            .configure(routes::routes::phone)
            .configure(routes::routes::image)
            .configure(routes::routes::user)
            .configure(routes::routes::gpt)
    })
    .bind((server_ip, server_port))?
    .run()
    .await?;

    Ok(())
}
