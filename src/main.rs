#[macro_use]
extern crate log;

use actix_web::{ HttpServer, App };
use actix_web::middleware::Logger;
use std::{env, io};
use crate::common::constants;

mod server;
mod common;
mod middleware;
mod helpers;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    info!("Starting...");

    dotenv::dotenv().ok();
    for (key, val) in env::vars() {
        debug!("{}: {}", key, val);
    }

    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .configure(server::load_services))
        .bind(env::var(constants::LISTEN_ADDR).unwrap().as_str())?
        .run()
        .await
}
