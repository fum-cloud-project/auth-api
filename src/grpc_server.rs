extern crate actix;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde;
mod grpc;
use grpc::auth::auth_service_server::{AuthService, AuthServiceServer};
use grpc::auth::{resource::Method, Access, JsonWebToken, Resource, User};
use tonic::{transport::Server, Request, Response, Status};

mod actors;
mod api_handlers;
mod bootstrap_utils;
mod cache_schemas;
mod db_schemas;
mod middlewares;
mod state;
mod utils;
//local modules
use crate::actors::database::resources::GetResource;
use crate::actors::database::users::GetUserWithId;
use crate::api_handlers::auth::{
    refresh::refresh, sign_in::sign_in, sign_out::sign_out, sign_up::sign_up,
};
use crate::db_schemas::resources::Method as IMethod;
use crate::middlewares::rbac;
use crate::state::AppState;
use crate::utils::tokens::verify_token;
use crate::utils::tokens::verify_token_and_get_user_id;
use bootstrap_utils::add_resources::add_resources;
//external modules
use crate::actix::Addr;
use crate::actors::cache::CacheActor;
use crate::actors::database::DbActor;
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

#[derive(Debug)]
pub struct Auth {
    pub db: Addr<DbActor>,
    pub cache: Addr<CacheActor>,
    pub salt: String,
    pub secret: String,
}

#[tonic::async_trait]
impl AuthService for Auth {
    async fn has_access(&self, request: Request<Resource>) -> Result<Response<Access>, Status> {
        let inner_msg = request.into_inner();
        let path = inner_msg.path;
        let db = self.db.clone();
        let cache = self.cache.clone();
        let bearer_tok = inner_msg.jwt;
        let secret: String = self.secret.chars().collect();
        let method = inner_msg.method;
        let method = if Method::Get as i32 == method {
            IMethod::GET
        } else if Method::Connect as i32 == method {
            IMethod::CONNECT
        } else if Method::Delete as i32 == method {
            IMethod::DELETE
        } else if Method::Options as i32 == method {
            IMethod::OPTIONS
        } else if Method::Patch as i32 == method {
            IMethod::PATCH
        } else if Method::Post as i32 == method {
            IMethod::POST
        } else if Method::Put as i32 == method {
            IMethod::PUT
        } else if Method::Trace as i32 == method {
            IMethod::TRACE
        } else {
            IMethod::INVALID
        };
        if let IMethod::INVALID = method {
            return Err(Status::invalid_argument("Invalid method"));
        } else if path.is_empty() {
            return Err(Status::invalid_argument("Empty path"));
        }
        let get_url = GetResource {
            path: path.to_string(),
            method: method,
        };
        let db = db.clone();
        let ret = match db.send(get_url).await {
            Ok(Ok(Ok(res))) => {
                let mut access_obj = Access { has_access: true };
                if res.access > 0 {
                    if let Ok(ids) = verify_token(bearer_tok, secret, cache.clone()).await {
                        if ids.1 < res.access {
                            access_obj.has_access = false;
                        } else {
                            access_obj.has_access = true;
                        }
                    } else {
                        access_obj.has_access = false;
                    }
                } else {
                    access_obj.has_access = true;
                }
                Ok(Response::new(access_obj))
            }
            Ok(Err(c)) if c == 0 => {
                let mut access_obj = Access { has_access: true };
                access_obj.has_access = false;
                Ok(Response::new(access_obj))
            }
            _ => Err(Status::internal("Something went wrong.")),
        };
        return ret;
    }
    async fn get_user(&self, request: Request<JsonWebToken>) -> Result<Response<User>, Status> {
        println!("Got a request: {:?}", request);
        let secret: String = self.secret.chars().collect();
        let _id = match verify_token_and_get_user_id(request.into_inner().jwt, secret).await {
            Ok(_id) => _id,
            _ => {
                return Err(Status::invalid_argument("Invalid jwt"));
            }
        };
        let db = self.db.clone();
        match db.send(GetUserWithId { _id }).await {
            Ok(Ok(Ok(res))) => {
                return Ok(Response::new(User {
                    id: res._id.to_hex(),
                    first_name: res.first_name,
                    last_name: res.last_name,
                    email: res.email,
                    access_level: res.access_level,
                }));
            }
            Ok(Err(e)) if e == 0 => return Err(Status::invalid_argument("Invalid jwt")),
            _ => {
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
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
    let auth_server = Auth {
        db: db_addr.clone(),
        cache: cache_addr.clone(),
        salt: dotenv!("SALT_STR").to_string(),
        secret: dotenv!("SECRET").to_string(),
    };

    Server::builder()
        .add_service(AuthServiceServer::new(auth_server))
        .serve(addr)
        .await?;

    Ok(())
}
