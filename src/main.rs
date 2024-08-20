use actix_web::{web, HttpServer, HttpResponse, App};
use sqlx::postgres::{PgPool, PgPoolOptions, PgQueryResult}; 
use serde::{Serialize, Deserialize};

#[derive(Clone)]
struct AppState {
    pool: PgPool
}

#[derive(Serialize, Deserialize)]
struct Phone {
    id: i32,
    model: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const DB_URL: &str = "postgres://admin:admin@localhost:5432/ci_db";

    let pool: PgPool = PgPoolOptions::new()
    .max_connections(10)
    .connect(DB_URL)
    .await
    .unwrap();

    let app_state = AppState {pool};

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(app_state.clone()))
        .route("/", web::get().to(root))
        .route("/get/{phone_id}", web::get().to(get_phone))
        .route("/delete/{phone_id}", web::delete().to(delete_phone))
        .route("/get", web::get().to(get_phones))
    }).bind(("localhost", 8000))?
    .run()
    .await?;

    Ok(())
}

async fn root() -> String {
    "Server is up and running".to_string()
}

async fn get_phone(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
    let phone_id: usize = path.into_inner();

    let phone: sqlx::Result<Phone> = sqlx::query_as!(
        Phone,
        "SELECT id, model FROM phones WHERE id = $1",
        phone_id as i64,
    ).fetch_one(&app_state.pool).await;

    match phone {
        Ok(phone) => HttpResponse::Ok().json(phone),
        Err(_) => HttpResponse::BadRequest().into()
    }
}

async fn delete_phone(path: web::Path<usize>, app_state: web::Data<AppState>) -> HttpResponse {
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

async fn get_phones(app_state: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(
        Phone,
        "SELECT id, model FROM phones"
    ).fetch_all(&app_state.pool).await {
        Ok(phones) => HttpResponse::Ok().json(phones),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
