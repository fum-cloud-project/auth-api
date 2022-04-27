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
mod cache_schemas;
mod db_schemas;
mod middlewares;
mod state;
mod utils;
//local modules
use crate::actors::cache::CacheActor;
use crate::actors::database::DbActor;
use bootstrap_utils::add_resources::add_resources;
//external modules
use actix::Actor;
use actix::Addr;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};

//structs
pub struct AppState {
    pub db: Addr<DbActor>,
    pub cache: Addr<CacheActor>,
    pub salt: String,
    pub secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    let cache_url = dotenv!("REDIS_URL");
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
    match add_resources(db_addr.clone()).await {
        Err(_) => {
            panic!("Adding resources failed");
        }
        _ => {}
    };

    let cc = redis::Client::open(cache_url).unwrap();
    let actor_cache = if let Ok(cm) = cc.get_tokio_connection_manager().await {
        cm
    } else {
        panic!("Init failed.");
    };
    let actor_cache = actors::cache::CacheActor(actor_cache);
    let cache_addr = actor_cache.start();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                cache: cache_addr.clone(),
                salt: dotenv!("SALT_STR").to_string(),
                secret: dotenv!("SECRET").to_string(),
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
