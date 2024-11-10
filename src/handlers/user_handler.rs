use actix_web::{web, HttpResponse};

use crate::{models::user::UserData, modules::hash_password::HashPassword, state::AppState};

pub struct UserHandler;

impl UserHandler {
    pub async fn create_user(
        app_state: web::Data<AppState>,
        user_data: web::Json<UserData>,
    ) -> HttpResponse {
        match HashPassword::hash_password(&user_data.password) {
            Ok(hash_password) => {
                match sqlx::query_file!(
                    "src/queries/insert_user.sql",
                    &user_data.username,
                    &user_data.email,
                    hash_password,
                )
                .execute(&app_state.pool)
                .await
                {
                    Ok(_) => HttpResponse::Created().into(),
                    Err(_) => HttpResponse::BadRequest().into(),
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Hash password error"),
        }
    }
}
