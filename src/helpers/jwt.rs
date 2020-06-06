use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData};
use chrono::{DateTime, Utc, Timelike, Duration};
use crate::helpers::errors;
use jsonwebtoken::errors::Error;
use crate::helpers::errors::SyncifyError;
use std::borrow::Borrow;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: DateTime<Utc>, // Required (validate_exp defaults to true in validation). Expiration time
    iat: DateTime<Utc>,  // Optional. Issued at
    uid: String,
}

pub fn generate_token(uid: &str) -> errors::Result<String> {
    let now = Utc::now();
    let my_claims = Claims {
        exp: now + Duration::hours(1),
        iat: now,
        uid: uid.into_string(),
    };

    match encode(&Header::new(Algorithm::HS512), &my_claims,
                 &EncodingKey::from_secret("secret".as_ref())) { // TODO: dotenv-ify
        Ok(str) => { Ok(str) },
        Err(err) => { Err(SyncifyError::InternalServer) },
    }
}

pub fn validate_token(token: &str) -> errors::Result<Claims> {
    match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()),
                     &Validation::new(Algorithm::HS512)) {
        Ok(result) => { Ok(result.claims) },
        Err(err) => { Err(SyncifyError::InternalServer) },
    }
}
