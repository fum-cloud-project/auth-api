extern crate actix;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

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
use crate::api_handlers::users::{
    create::create, delete::delete_admin, delete_acc::delete, get_many::get_many,
    get_my_acc::get_my_acc, get_one::get_one, update::update_admin, update_acc::update,
};
use crate::bootstrap_utils::setup_logger::setup_logger;
use crate::middlewares::rbac;
use crate::state::AppState;
use bootstrap_utils::add_resources::add_resources;
//external modules
use actix::Actor;
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{get, http, middleware, web::Data, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::path::Path;
use std::sync::Arc;

lazy_static! {
    static ref DOCS: std::path::PathBuf = {
        let path = dotenv!("DOCS_PATH");
        std::path::PathBuf::from(path)
    };
}

#[get("")]
async fn index() -> NamedFile {
    NamedFile::open_async(DOCS.clone()).await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    let cache_url = dotenv!("REDIS_URL");
    let log_file = dotenv!("API_LOG_FILE");
    let admin_email = dotenv!("ADMIN_EMAIL");
    let admin_password = dotenv!("ADMIN_PASSWORD");
    let secret = Arc::new(dotenv!("SECRET").as_bytes());
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

    let cc = redis::Client::open(cache_url).unwrap();
    let actor_cache = if let Ok(cm) = cc.get_tokio_connection_manager().await {
        cm
    } else {
        panic!("Init failed.");
    };
    let actor_cache = actors::cache::CacheActor(actor_cache);
    let cache_addr = actor_cache.start();

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
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(rbac::Rbac {
                db: db_addr.clone(),
                cache: cache_addr.clone(),
                secret: secret.clone(),
            })
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(
                actix_web::web::scope("/api")
                    .service(
                        actix_web::web::scope("/auth")
                            .service(sign_out)
                            .service(refresh)
                            .service(sign_in)
                            .service(sign_up),
                    )
                    .service(
                        actix_web::web::scope("/users")
                            .service(create)
                            .service(update)
                            .service(update_admin)
                            .service(get_my_acc)
                            .service(get_one)
                            .service(get_many)
                            .service(delete)
                            .service(delete_admin),
                    )
                    .service(actix_web::web::scope("/docs").service(index)),
            )
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                cache: cache_addr.clone(),
                salt: salt.clone(),
                secret: secret.clone(),
            }))
    })
    .bind((
        dotenv!("API_SERVER_ADDRESS"),
        dotenv!("API_SERVER_PORT").parse().unwrap(),
    ))?
    .workers(dotenv!("WORKET_THREAD_NUM").parse().unwrap())
    .run()
    .await
}
