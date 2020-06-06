use crate::common::constants;
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub real_name: Option<String>,
}

impl User {
    pub fn set_password(&mut self, raw: &str) {
        let salt = env::var(constants::PASSWORD_SALT).unwrap();
        let config = Config::default();
        self.password = argon2::hash_encoded(raw.as_ref(), salt.as_ref(), &config).unwrap();
    }

    pub fn compare_password(&self, raw_passwd: &str) -> bool {
        return argon2::verify_encoded(self.password.as_str(), raw_passwd.as_ref()).unwrap();
    }
}
