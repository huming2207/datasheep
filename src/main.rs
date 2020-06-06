#[macro_use]
extern crate log;
extern crate argon2;

use crate::common::constants;
use crate::middleware::jwt_validator;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::{env, io};

mod common;
mod helpers;
mod middleware;
mod models;
mod requests;
mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    info!("Starting...");

    dotenv::dotenv().ok();
    for (key, val) in env::vars() {
        debug!("{}: {}", key, val);
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(jwt_validator::validator)
            .configure(server::load_services)
    })
    .bind(env::var(constants::LISTEN_ADDR).unwrap().as_str())?
    .run()
    .await
}
