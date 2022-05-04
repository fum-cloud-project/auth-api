extern crate actix;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde;
mod grpc;

mod actors;
mod api_handlers;
mod bootstrap_utils;
mod cache_schemas;
mod db_schemas;
mod middlewares;
mod state;
mod utils;
//local modules
use crate::bootstrap_utils::setup_logger::setup_logger;
use crate::grpc::auth_impl::Auth;
use bootstrap_utils::add_resources::add_resources;
use grpc::auth::auth_service_server::AuthServiceServer;
use tonic::transport::Server;
//external modules
use actix::Actor;
use mongodb::{options::ClientOptions, Client};
use std::path::Path;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = dotenv!("GRPC_SERVER_URL_AND_PORT").parse()?;
    let db_url = dotenv!("DATABASE_URL");
    let cache_url = dotenv!("REDIS_URL");
    let log_file = dotenv!("GRPC_LOG_FILE");
    let admin_email = dotenv!("ADMIN_EMAIL");
    let admin_password = dotenv!("ADMIN_PASSWORD");
    let salt = Arc::new(dotenv!("SALT_STR"));
    let resources = Path::new(dotenv!("RESOURCES"));
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
    match add_resources(
        db_addr.clone(),
        salt.clone(),
        resources,
        admin_email,
        admin_password,
    )
    .await
    {
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

    let auth_server = Auth {
        db: db_addr.clone(),
        cache: cache_addr.clone(),
        salt: Arc::new(dotenv!("SALT_STR")),
        secret: Arc::new(dotenv!("SECRET").as_bytes()),
    };

    Server::builder()
        .add_service(AuthServiceServer::new(auth_server))
        .serve(addr)
        .await?;

    Ok(())
}
