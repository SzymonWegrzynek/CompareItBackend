use actix_web::{http::header, web, HttpResponse};

use crate::{
    models::user::{CreateUserData, CreateUserResponse, SignInResponse, SignInUserData, User},
    modules::{hash_password::HashPassword, jwt_token::JwtToken},
    state::AppState,
};

pub struct LoginHandler;

impl LoginHandler {
    pub async fn create_user(
        app_state: web::Data<AppState>,
        payload: web::Json<CreateUserData>,
    ) -> HttpResponse {
        match HashPassword::hash_password(&payload.password) {
            Ok(hash_password) => {
                match sqlx::query_file!(
                    "src/queries/insert_user.sql",
                    &payload.username,
                    &payload.email,
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
        payload: web::Json<SignInUserData>,
    ) -> HttpResponse {
        let user = sqlx::query_file_as!(User, "src/queries/get_user.sql", &payload.email)
            .fetch_one(&app_state.pool)
            .await;

        let user = match user {
            Ok(record) => record,
            Err(_) => {
                return HttpResponse::Unauthorized().json(SignInResponse {
                    message: "Incorrect email or password".to_string(),
                });
            }
        };

        match HashPassword::verify_password(&payload.password, &user.password) {
            Ok(true) => {
                match JwtToken::encode_token(
                    user.user_id.try_into().unwrap(),
                    user.role.to_string(),
                    &app_state,
                ) {
                    Ok(token) => HttpResponse::Ok()
                        .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
                        .json(SignInResponse {
                            message: "Successfully logged in".to_string(),
                        }),
                    Err(_) => HttpResponse::InternalServerError().json(SignInResponse {
                        message: "Error generating token".to_string(),
                    }),
                }
            }
            Ok(false) => HttpResponse::Unauthorized().json(SignInResponse {
                message: "Incorrect email or password".to_string(),
            }),
            Err(_) => HttpResponse::Unauthorized().json(SignInResponse {
                message: "Sign in error".to_string(),
            }),
        }
    }
}
