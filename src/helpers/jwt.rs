use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Claims {
    email: String,
    id: i32,
    exp: i64,
}

#[derive(Deserialize, Serialize)]
pub struct TokenUser {
    pub email: String,
    pub id: i32,
}

pub fn get_jwt(email: String, id: i32) -> Result<String, String> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = encode(
        &Header::default(),
        &Claims {
            email: email,
            id: id,
            exp: (Utc::now() + Duration::hours(8)).timestamp(),
        },
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|e| e.to_string());

    return token;
}

pub fn decode_jwt(token: &str) -> Result<TokenUser, String> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<TokenUser>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e.to_string()),
    }
}
