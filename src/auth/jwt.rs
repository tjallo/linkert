use std::fmt::Error;

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, models::user::User};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: i64,
    exp: usize,
    nbf: usize,
    iat: usize,
}

const JWT_EXPIRY_IN_MINUTES: i64 = 60;

pub fn jwt_encode(state: AppState, user: &User) -> Option<String> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let nbf = iat;
    let exp = (now + chrono::Duration::minutes(JWT_EXPIRY_IN_MINUTES)).timestamp() as usize;

    let claims = Claims {
        sub: user.username.clone(),
        user_id: user.id,
        iat,
        nbf,
        exp,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config_vars.jwt_secret.as_ref()),
    ) {
        Ok(jwt) => Some(jwt),
        Err(_) => None,
    }
}
