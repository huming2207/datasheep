use crate::models::mongo_doc_model::MongoDocModel;
use crate::models::resp_body::ResponseBody;
use crate::models::user::User;
use actix_web::{web, HttpResponse};
use bson::doc;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::helpers::jwt::generate_token_from_user;

#[derive(Deserialize, Serialize, Validate)]
pub struct UserLoginForm {
    #[serde(default)]
    #[validate(length(min = 1))]
    username: String,
    #[serde(default)]
    #[validate(length(min = 5))]
    password: String,
}

pub async fn login(
    user_form: web::Form<UserLoginForm>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    let login_user = user_form.into_inner();
    let validate_ret: Result<(), ValidationErrors>  = login_user.validate();
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
