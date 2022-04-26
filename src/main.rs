extern crate actix;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate mongodb;

mod actors;
mod api_handlers;
mod bootstrap_utils;
mod db_schemas;
mod middlewares;
//local modules
use crate::actors::database::DbActor;

//external modules
use actix::Actor;
use actix::Addr;
use actix::SyncArbiter;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};

//structs
pub struct AppState {
    pub db: Addr<DbActor>,
    pub salt: String,
    pub secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    let client_options = match ClientOptions::parse(db_url).await {
        Ok(co) => co,
        _ => {
            panic!("could not parse db uri");
        }
    };
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("cloudFUMAuthDB");
    let actor_db = actors::database::DbActor(db);
    let db_addr = actor_db.start();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                salt: dotenv!("SALT_STR").to_string(),
                secret: dotenv!("SECRET").to_string(),
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
