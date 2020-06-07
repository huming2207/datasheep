#[macro_use]
extern crate log;
extern crate argon2;

#[macro_use]
extern crate bson;

#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;
extern crate serde_json;

use crate::common::constants;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use mongodb::Client;
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

    let db_client = match Client::with_uri_str(env::var(constants::DB_URL).unwrap().as_str()).await
    {
        Ok(client) => Ok(client),
        Err(err) => Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "Failed when connecting to database",
        )),
    }?;

    let db_database = db_client.database(env::var(constants::DB_NAME).unwrap().as_str());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(db_database.clone())
            .configure(server::load_services)
    })
    .bind(env::var(constants::LISTEN_ADDR).unwrap().as_str())?
    .run()
    .await
}
