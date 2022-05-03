use crate::actix::Addr;
use crate::actors::cache::CacheActor;
use crate::actors::database::DbActor;
use std::sync::Arc;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub cache: Addr<CacheActor>,
    pub salt: Arc<&'static str>,
    pub secret: Arc<&'static [u8]>,
}
