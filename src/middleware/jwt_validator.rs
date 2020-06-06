use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::dev::{ServiceRequest, PayloadStream, Payload};
use std::io;
use crate::helpers::jwt;
use actix_web::{dev, FromRequest, HttpRequest};
use futures::future::{ok, err, Ready};
use actix_web::http::HeaderValue;
use crate::helpers::errors::SyncifyError;
use actix_web::error::ErrorUnauthorized;
use log::Level::Error;
use crate::helpers::jwt::{validate_token, JwtClaims};
use std::borrow::Borrow;

#[derive(Debug, Deserialize)]
struct UserContext {
    id: String
}

impl FromRequest for UserContext {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            None => {
                err(ErrorUnauthorized("No Authorization found"))
            },
            Some(value) => {
                let bearer_token = value.to_str()?;
                if bearer_token.starts_with("Bearer") || bearer_token.starts_with("bearer") {
                    let token = bearer_token[6..bearer_token.len()].trim();
                    match validate_token(token) {
                        Ok(claim) => {
                            ok(UserContext{ id: claim.uid.clone() })
                        },
                        Err(_) => {
                            err(ErrorUnauthorized("Invalid token"))
                        },
                    }
                } else {
                    err(ErrorUnauthorized("Token is not a bearer token"))
                }
            },
        }
    }
}