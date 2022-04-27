use crate::actix::{Actor, Context};
use r2d2::Pool;
use redis::aio::ConnectionManager;
pub mod tokens;

pub struct CacheActor(pub ConnectionManager);

impl Actor for CacheActor {
    type Context = Context<Self>;
}
