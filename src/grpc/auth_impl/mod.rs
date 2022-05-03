//local modules
use crate::actors::database::resources::GetResource;
use crate::actors::database::users::GetUserWithId;
use crate::db_schemas::resources::Method as IMethod;
use crate::grpc::auth::auth_service_server::AuthService;
use crate::grpc::auth::{resource::Method, Access, JsonWebToken, Resource, User};
use crate::utils::tokens::verify_token;
use crate::utils::tokens::verify_token_and_get_user_id;
use tonic::{Request, Response, Status};
//external modules
use crate::actix::Addr;
use crate::actors::cache::CacheActor;
use crate::actors::database::DbActor;
use std::sync::Arc;

#[derive(Debug)]
pub struct Auth {
    pub db: Addr<DbActor>,
    pub cache: Addr<CacheActor>,
    pub salt: Arc<&'static str>,
    pub secret: Arc<&'static [u8]>,
}

#[tonic::async_trait]
impl AuthService for Auth {
    async fn has_access(&self, request: Request<Resource>) -> Result<Response<Access>, Status> {
        let inner_msg = request.into_inner();
        let path = inner_msg.path;
        let db = self.db.clone();
        let cache = self.cache.clone();
        let bearer_tok = inner_msg.jwt;
        let secret = self.secret.clone();
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
        let secret = self.secret.clone();
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
