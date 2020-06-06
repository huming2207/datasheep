use crate::common::constants;
use chrono::{DateTime, Duration, Timelike, Utc};
use jsonwebtoken::errors::{Error, Result};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    exp: DateTime<Utc>, // Required (validate_exp defaults to true in validation). Expiration time
    iat: DateTime<Utc>,
    pub uid: String,
}

pub fn generate_token(uid: &str) -> Result<String> {
    let now = Utc::now();
    let my_claims = JwtClaims {
        exp: now + Duration::hours(1),
        iat: now,
        uid: uid.to_string(),
    };

    match encode(
        &Header::new(Algorithm::HS512),
        &my_claims,
        &EncodingKey::from_secret(env::var(constants::JWT_SECRET).unwrap().as_bytes()),
    ) {
        Ok(str) => Ok(str),
        Err(err) => Err(err),
    }
}

pub fn validate_token(token: &str) -> Result<JwtClaims> {
    match decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(env::var(constants::JWT_SECRET).unwrap().as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(result) => Ok(result.claims),
        Err(err) => Err(err),
    }
}
