use crate::actix::Addr;
use crate::actors::cache::CacheActor;
use crate::actors::database::DbActor;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub cache: Addr<CacheActor>,
    pub salt: String,
    pub secret: String,
}
