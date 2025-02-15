use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http::header::{HeaderValue, AUTHORIZATION},
    web, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::{models::auth::Claims, state::AppState};

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
    pub role: String,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let auth_header: Option<&HeaderValue> = req.headers().get(AUTHORIZATION);

        if auth_header.is_none() {
            return ready(Err(ErrorUnauthorized("No authentication token sent")));
        }

        let auth_token: String = auth_header.unwrap().to_str().unwrap_or("").to_string();

        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized(
                "Authentication token has foreign chars",
            )));
        }

        let app_state = match req.app_data::<web::Data<AppState>>() {
            Some(state) => state,
            None => return ready(Err(ErrorUnauthorized("Missing app state"))),
        };

        let jwt_secret = &app_state.jwt_secret;

        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
                role: token.claims.role,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Invalid authentication token sent"))),
        }
    }
}
