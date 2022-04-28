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
use crate::middlewares::rbac;
use crate::state::AppState;
use bootstrap_utils::add_resources::add_resources;
//external modules
use actix::Actor;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use fern::colors::{Color, ColoredLevelConfig};
use mongodb::{options::ClientOptions, Client};

fn setup_logger(file_path: &str) -> Result<(), fern::InitError> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    let colors_level = colors_line.clone().info(Color::Green);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .chain(fern::log_file(file_path)?)
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    let cache_url = dotenv!("REDIS_URL");
    let log_file = dotenv!("LOG_FILE");
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

    setup_logger(log_file).expect("Logger initialization failed.");
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
