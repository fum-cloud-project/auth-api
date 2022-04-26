use crate::actix::{Actor, Context};
use mongodb::Database;
pub mod users;
pub mod resources;

pub struct DbActor(pub Database);

impl Actor for DbActor {
    type Context = Context<Self>;
}
