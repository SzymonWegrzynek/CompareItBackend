use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};

use crate::extractors::authentication_token::AuthenticationToken;
use crate::models::auth::{Claims, DecodeBody, DecodeResponse, EncodeResponse, Response};
use crate::state::AppState;

pub struct JwtToken;

impl JwtToken {
    pub async fn encode_token(
        path: web::Path<usize>,
        app_state: web::Data<AppState>,
    ) -> HttpResponse {
        let id: usize = path.into_inner();
        let exp: usize = (Utc::now() + Duration::days(7)).timestamp() as usize;
        let claims: Claims = Claims { id, exp };
        let token: String = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(app_state.jwt_secret.as_str().as_ref()),
        )
        .unwrap();

        HttpResponse::Ok().json(EncodeResponse {
            message: "Successfully created account".to_owned(),
            token,
        })
    }

    pub async fn decode_token(
        body: web::Json<DecodeBody>,
        app_state: web::Data<AppState>,
    ) -> HttpResponse {
        let decoded: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &body.token,
            &DecodingKey::from_secret(app_state.jwt_secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decoded {
            Ok(token) => HttpResponse::Ok().json(DecodeResponse {
                message: "Successfully logged in".to_string(),
                id: token.claims.id,
            }),
            Err(e) => HttpResponse::BadRequest().json(Response {
                message: e.to_string(),
            }),
        }
    }

    pub async fn protected(auth_token: AuthenticationToken) -> HttpResponse {
        println!("{}", auth_token.id);

        HttpResponse::Ok().json(Response {
            message: "Authorized".to_owned(),
        })
    }
}
