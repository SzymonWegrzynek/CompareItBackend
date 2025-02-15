use actix_web::{web, HttpResponse};

use crate::{
    models::token::{ProtectRoute, ProtectRouteResponse, VerifyToken},
    modules::jwt_token::JwtToken,
    state::AppState,
};

pub struct TokenHandler;

impl TokenHandler {
    pub async fn verify_token(
        app_state: web::Data<AppState>,
        payload: web::Json<VerifyToken>,
    ) -> HttpResponse {
        match JwtToken::decode_token(&payload.token, &app_state) {
            Ok(_) => HttpResponse::Ok().json("Token is valid"),
            Err(_) => HttpResponse::Unauthorized().json("Invalid token"),
        }
    }

    pub async fn protect_route(
        app_state: web::Data<AppState>,
        payload: web::Json<ProtectRoute>,
    ) -> HttpResponse {
        let token = &payload.token;

        match JwtToken::decode_token(token, &app_state) {
            Ok(decoded_token) => {
                let role = decoded_token.claims.role;
                HttpResponse::Ok().json(ProtectRouteResponse { role })
            }
            Err(_) => HttpResponse::Unauthorized().json("Invalid or expired token"),
        }
    }
}
