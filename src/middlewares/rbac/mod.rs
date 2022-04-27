use crate::actix::Addr;
use crate::actors::cache::CacheActor;
use crate::actors::database::resources::GetResource;
use crate::actors::database::DbActor;
use crate::db_schemas::resources::{Method, Resources};
use crate::utils::tokens::verify_token;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthExtractor;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct Rbac {
    pub db: Addr<DbActor>,
    pub cache: Addr<CacheActor>,
    pub secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for Rbac
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RbacMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RbacMiddleware {
            service: service,
            db: self.db.clone(),
            cache: self.cache.clone(),
            secret: self.secret.chars().collect(),
        }))
    }
}

pub struct RbacMiddleware<S> {
    service: S,
    db: Addr<DbActor>,
    cache: Addr<CacheActor>,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for RbacMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
        let db = self.db.clone();
        let cache = self.cache.clone();
        let bearer_tok = BearerAuth::from_service_request(&req);
        let secret: String = self.secret.chars().collect();
        let method = req.method();
        let method = match *method {
            actix_web::http::Method::GET => Method::GET,
            actix_web::http::Method::CONNECT => Method::CONNECT,
            actix_web::http::Method::DELETE => Method::DELETE,
            actix_web::http::Method::OPTIONS => Method::OPTIONS,
            actix_web::http::Method::PATCH => Method::PATCH,
            actix_web::http::Method::POST => Method::POST,
            actix_web::http::Method::PUT => Method::PUT,
            actix_web::http::Method::TRACE => Method::TRACE,
            _ => Method::INVALID,
        };
        let get_url = GetResource {
            path: path.to_string(),
            method: method,
        };
        let fut = self.service.call(req);

        Box::pin(async move {
            let db = db.clone();
            match db.send(get_url).await {
                Ok(Ok(Ok(res))) => {
                    if res.access == 0 {
                        match fut.await {
                            Ok(res) => {
                                return Ok(res);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    let bearer_tok: String = match bearer_tok.await {
                        Ok(val) => val.token().chars().filter(|c| !c.is_whitespace()).collect(),
                        _ => {
                            return Err(actix_web::error::ErrorUnauthorized(
                                "You don't have access to this resource",
                            ));
                        }
                    };
                    let ids = match verify_token(bearer_tok, secret, cache.clone()).await {
                        Ok(ids) => ids,
                        _ => {
                            return Err(actix_web::error::ErrorUnauthorized(
                                "You don't have access to this resource",
                            ));
                        }
                    };
                    if ids.1 < res.access {
                        return Err(actix_web::error::ErrorUnauthorized(
                            "You don't have access to this resource",
                        ));
                    } else {
                        match fut.await {
                            Ok(res) => {
                                return Ok(res);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                _ => {
                    return Err(actix_web::error::ErrorInternalServerError(
                        "Something went wrong.",
                    ));
                }
            }
        })
    }
}
