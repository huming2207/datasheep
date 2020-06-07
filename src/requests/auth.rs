use crate::helpers::jwt::{generate_token_from_user};
use crate::models::mongo_doc_model::MongoDocModel;
use crate::models::resp_body::ResponseBody;
use crate::models::user::User;
use actix_web::{web, HttpResponse};
use bson::doc;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Deserialize, Serialize, Validate)]
pub struct AuthUser {
    #[serde(default)]
    #[validate(length(min = 1))]
    username: String,
    #[serde(default)]
    #[validate(length(min = 5))]
    password: String,
    #[serde(default)]
    #[validate(email)]
    email: String,
}

pub async fn register(
    user_form: web::Form<AuthUser>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    let new_user = user_form.into_inner();
    let validate_ret: Result<(), ValidationErrors> = new_user.validate();
    match validate_ret {
        Ok(_) => (),
        Err(error) => {
            info!("Invalid content submitted: {}", error);
            return HttpResponse::BadRequest()
                .json(ResponseBody::new("Invalid content submitted", error));
        }
    }

    let collection = db.collection(User::collection_name());
    let find_ret = collection
        .find_one(doc! {"username": new_user.username.clone()}, None)
        .await;
    match find_ret {
        Ok(ret) => match ret {
            Some(_) => {
                HttpResponse::NotAcceptable().json(ResponseBody::new("User already exist", ""))
            }
            None => {
                let user = User {
                    id: bson::oid::ObjectId::new(),
                    username: new_user.username.clone(),
                    email: new_user.email.clone(),
                    password: "".to_string(),
                };

                let user_bson = bson::to_bson(&user).unwrap();
                if let bson::Bson::Document(document) = user_bson {
                    let ret = collection.insert_one(document, None).await;
                    match ret {
                        Ok(_) => {
                            HttpResponse::Ok().json(ResponseBody::new("User created", ""))
                        }
                        Err(error) => {
                            error!("Failed when saving user to database: {}", error);
                            HttpResponse::InternalServerError()
                                .json(ResponseBody::new("Failed when saving user to database", ""))
                        }
                    }
                } else {
                    HttpResponse::InternalServerError()
                        .json(ResponseBody::new("Failed to serialize user", ""))
                }
            }
        },
        Err(error) => {
            error!("Failed to check user existence: {}", error);
            HttpResponse::InternalServerError()
                .json(ResponseBody::new("Failed to check user existence", ""))
        }
    }
}

pub async fn login(
    user_form: web::Form<AuthUser>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    let login_user = user_form.into_inner();
    let validate_ret = login_user.validate();
    match validate_ret {
        Ok(_) => (),
        Err(error) => {
            return HttpResponse::BadRequest()
                .json(ResponseBody::new("Invalid content submitted", error));
        }
    }

    let collection = db.collection(User::collection_name());
    let find_ret = collection
        .find_one(doc! {"username": login_user.username}, None)
        .await;
    match find_ret {
        Ok(ret) => match ret {
            Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
                Ok(result) => {
                    let user: User = result;
                    if user.compare_password(login_user.password.as_str()) {
                        let token_result = generate_token_from_user(&user);
                        match token_result {
                            Ok(token_str) => {
                                HttpResponse::Ok().json(ResponseBody::new("OK", token_str))
                            }
                            Err(err) => HttpResponse::Unauthorized()
                                .json(ResponseBody::new("Unauthorised", err.to_string())),
                        }
                    } else {
                        HttpResponse::NotFound().json(ResponseBody::new("User not found", ""))
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(ResponseBody::new(
                    "Invalid user returned from database",
                    error.to_string(),
                )),
            },
            None => HttpResponse::NotFound().json(ResponseBody::new("User not found", "")),
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(ResponseBody::new("Failed to find user", ""))
        }
    }
}
