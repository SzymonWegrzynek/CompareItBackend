use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};

use crate::{
    extractors::authentication_token::AuthenticationToken, models::auth::Claims, state::AppState,
};

pub struct JwtToken;

impl JwtToken {
    pub fn encode_token(id: usize, app_state: &AppState) -> Result<String, JwtError> {
        let exp: usize = (Utc::now() + Duration::days(1)).timestamp() as usize;
        let claims: Claims = Claims { id, exp };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(app_state.jwt_secret.as_str().as_ref()),
        )
    }

    pub fn decode_token(token: &str, app_state: &AppState) -> Result<TokenData<Claims>, JwtError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(app_state.jwt_secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        )
    }

    pub async fn protected(auth_token: &AuthenticationToken) -> Result<String, String> {
        println!("{}", auth_token.id);
        Ok("Authorized".to_owned())
    }
}
