extern crate actix;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde;

mod actors;
mod api_handlers;
mod bootstrap_utils;
mod cache_schemas;
mod db_schemas;
mod middlewares;
mod state;
mod utils;
//local modules
use crate::api_handlers::auth::{
    refresh::refresh, sign_in::sign_in, sign_out::sign_out, sign_up::sign_up,
};
use crate::bootstrap_utils::setup_logger::setup_logger;
use crate::middlewares::rbac;
use crate::state::AppState;
use bootstrap_utils::add_resources::add_resources;
//external modules
use actix::Actor;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use mongodb::{options::ClientOptions, Client};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    let cache_url = dotenv!("REDIS_URL");
    let log_file = dotenv!("API_LOG_FILE");
    setup_logger(log_file).expect("Logger initialization failed.");
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

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(rbac::Rbac {
                db: db_addr.clone(),
                cache: cache_addr.clone(),
                secret: dotenv!("SECRET").to_string(),
            })
            .service(
                actix_web::web::scope("/api").service(
                    actix_web::web::scope("/auth")
                        .service(sign_out)
                        .service(refresh)
                        .service(sign_in)
                        .service(sign_up),
                ),
            )
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                cache: cache_addr.clone(),
                salt: dotenv!("SALT_STR").to_string(),
                secret: dotenv!("SECRET").to_string(),
            }))
    })
    .bind((
        dotenv!("API_SERVER_ADDRESS"),
        dotenv!("API_SERVER_PORT").parse().unwrap(),
    ))?
    .workers(12)
    .run()
    .await
}
