use crate::helpers::jwt::validate_token;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

#[derive(Debug, Deserialize)]
struct UserContext {
    id: String,
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            req.extensions_mut().insert(UserContext {
                id: res.uid.clone(),
            });
            Ok(req)
        }
        Err(err) => Err(ErrorUnauthorized(err)),
    }
}
