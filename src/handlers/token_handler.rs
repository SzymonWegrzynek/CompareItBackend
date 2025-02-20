use actix_web::HttpResponse;

use crate::models::auth::AuthenticationToken;

pub struct TokenHandler;

impl TokenHandler {
    pub async fn extract_token(token: AuthenticationToken) -> HttpResponse {
        HttpResponse::Ok().json(format!("{}", token.role))
    }
}
