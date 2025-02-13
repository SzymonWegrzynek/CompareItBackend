use actix_web::{web, HttpResponse};

use crate::{
    extractors::authentication_token::AuthenticationToken,
    models::token::{ProtectRoute, VerifyToken},
    modules::jwt_token::JwtToken,
    state::AppState,
};

pub struct TokenHandler;

impl TokenHandler {
    pub async fn verify_token(
        app_state: web::Data<AppState>,
        json: web::Json<VerifyToken>,
    ) -> HttpResponse {
        match JwtToken::decode_token(&json.token, &app_state) {
            Ok(_) => HttpResponse::Ok().json("Token is valid"),
            Err(_) => HttpResponse::Unauthorized().json("Invalid token"),
        }
    }

    pub async fn protect_route(json: web::Json<ProtectRoute>) -> HttpResponse {
        let auth_token = AuthenticationToken {
            id: json.id.parse().unwrap(),
        };

        match JwtToken::protected(&auth_token).await {
            Ok(message) => HttpResponse::Ok().json(message),
            Err(_) => HttpResponse::Forbidden().json("Unauthorized"),
        }
    }
}
