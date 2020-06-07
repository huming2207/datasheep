use crate::common::constants;
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use std::env;
use crate::models::mongo_doc_model::MongoDocModel;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn set_password(&mut self, raw: &str) {
        let salt = env::var(constants::PASSWORD_SALT).unwrap();
        let config = Config::default();
        self.password = argon2::hash_encoded(raw.as_ref(), salt.as_bytes(), &config).unwrap();
    }

    pub fn compare_password(&self, raw: &str) -> bool {
        argon2::verify_encoded(self.password.as_str(), raw.as_bytes()).unwrap()
    }
}

impl MongoDocModel for User {
    fn collection_name() -> &'static str {
        "user"
    }
}