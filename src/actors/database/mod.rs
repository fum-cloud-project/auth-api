use crate::actix::{Actor, Context};
use mongodb::Database;
pub mod resources;
pub mod users;

pub struct DbActor(pub Database);

impl Actor for DbActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}
