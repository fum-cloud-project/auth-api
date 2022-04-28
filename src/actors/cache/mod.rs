use crate::actix::{Actor, Context};
use redis::aio::ConnectionManager;
pub mod tokens;

pub struct CacheActor(pub ConnectionManager);

impl Actor for CacheActor {
    type Context = Context<Self>;
}
