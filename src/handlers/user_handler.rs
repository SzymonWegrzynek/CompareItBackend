use actix_web::{web, HttpResponse};

use crate::{
    models::user::{CreateUserData, CreateUserResponse, SignInResponse, SignInUserData, User},
    modules::{hash_password::HashPassword, jwt_token::JwtToken},
    state::AppState,
};

pub struct UserHandler;

impl UserHandler {
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
        let result = sqlx::query_file!("src/queries/get_user_password.sql", &payload.email)
            .fetch_one(&app_state.pool)
            .await;

        let uid = sqlx::query_file!("src/queries/get_user_id.sql", &payload.email)
            .fetch_one(&app_state.pool)
            .await;

        let role = sqlx::query_file_as!(User, "src/queries/get_user_role.sql", &payload.email)
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

        match HashPassword::verify_password(&payload.password, &hashed_password) {
            Ok(true) => {
                let uid = match uid {
                    Ok(record) => record.user_id,
                    Err(_) => {
                        return HttpResponse::Unauthorized().json(SignInResponse {
                            message: "User id not found".to_string(),
                            token: "".to_string(),
                        });
                    }
                };

                let role = match role {
                    Ok(record) => record.role.to_string(),
                    Err(_) => {
                        return HttpResponse::Unauthorized().json(SignInResponse {
                            message: "User role not found".to_string(),
                            token: "".to_string(),
                        });
                    }
                };

                match JwtToken::encode_token(uid.try_into().unwrap(), role, &app_state) {
                    Ok(token) => HttpResponse::Ok().json(SignInResponse {
                        message: "Successfully logged in".to_string(),
                        token,
                    }),
                    Err(_) => HttpResponse::InternalServerError().json(SignInResponse {
                        message: "Token generate error".to_string(),
                        token: "".to_string(),
                    }),
                }
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
