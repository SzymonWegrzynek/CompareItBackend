use actix_web::{web, HttpResponse};

use crate::{
    handlers::auth_handler::JwtToken,
    models::user::{CreateUserData, CreateUserResponse, SignInResponse, SignInUserData},
    modules::hash_password::HashPassword,
    state::AppState,
};

pub struct UserHandler;

impl UserHandler {
    pub async fn create_user(
        app_state: web::Data<AppState>,
        user_data: web::Json<CreateUserData>,
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
                    Ok(_) => HttpResponse::Created().json(CreateUserResponse {
                        message: "User created successfully".to_string(),
                    }),
                    Err(_) => HttpResponse::BadRequest().json(CreateUserResponse {
                        message: "Failed to create user".to_string(),
                    }),
                }
            }
            Err(_) => HttpResponse::InternalServerError().json(CreateUserResponse {
                message: "Hash password error".to_string(),
            }),
        }
    }

    pub async fn signin_user(
        app_state: web::Data<AppState>,
        user_data: web::Json<SignInUserData>,
    ) -> HttpResponse {
        let result = sqlx::query_file!("src/queries/get_user_password.sql", &user_data.email)
            .fetch_one(&app_state.pool)
            .await;

        let uid = sqlx::query_file!("src/queries/get_user_id.sql", &user_data.email)
            .fetch_one(&app_state.pool)
            .await;

        let hashed_password = match result {
            Ok(record) => record.password,
            Err(_) => {
                return HttpResponse::Unauthorized().json(SignInResponse {
                    message: "Incorrect email or password".to_string(),
                    token: "".to_string(),
                })
            }
        };

        match HashPassword::verify_password(&user_data.password, &hashed_password) {
            Ok(true) => {
                let uid = match uid {
                    Ok(record) => record.user_id,
                    Err(_) => {
                        return HttpResponse::Unauthorized().json(SignInResponse {
                            message: "user_id not found".to_string(),
                            token: "".to_string(),
                        });
                    }
                };

                let token =
                    JwtToken::encode_token(uid.try_into().unwrap(), app_state.clone()).await;

                token
            }
            Ok(false) => HttpResponse::Unauthorized().json(SignInResponse {
                message: "Incorrect email or password".to_string(),
                token: "".to_string(),
            }),
            Err(_) => HttpResponse::Unauthorized().json(SignInResponse {
                message: "Error".to_string(),
                token: "".to_string(),
            }),
        }
    }
}
