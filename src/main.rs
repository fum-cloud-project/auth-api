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

//external modules
use crate::actix::Actor;
use actix::SyncArbiter;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    let mut client_options = match ClientOptions::parse(db_url).await {
        Ok(co) => co,
        _ => {
            panic!("could not parse db uri");
        }
    };
    let client = Client::with_options(client_options).unwrap();
    if let Ok(dbs) = client.list_database_names(None, None).await {
        for db_name in dbs {
            println!("{}", db_name);
        }
    }
    let db = client.database("cloudFUMAuthDB");
    if let Ok(cs) = db.list_collection_names(None).await {
        println!("Created database");
        for cs_name in cs {
            println!("{}", cs_name);
        }
    }
    let actor_db = actors::database::DbActor(db);
    let db_addr = actor_db.start();
    match db_addr
        .clone()
        .send(actors::database::users::CreateUser {
            first_name: String::from("Tooraj"),
            last_name: String::from("Taraz"),
            email: String::from("tooraj.info@gmail.com"),
            password: String::from("12345678"),
            access_level: 100,
        })
        .await
    {
        Ok(res) => {
            println!("ok {:?}", res);
        }
        Err(e) => {
            println!(" err {:?}", e);
        }
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || App::new().wrap(Logger::default()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
