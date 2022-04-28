use crate::actix::ResponseFuture;
use crate::actix::{Handler, Message};
use crate::actors::database::DbActor;
use crate::db_schemas::resources::{Method, Resources};
use mongodb::bson::doc;
use mongodb::error::Error;

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct CreateOrUpdateResource {
    pub method: Method,
    pub path: String,
    pub access: i32,
}

#[derive(Message)]
#[rtype(result = "Result<Result<Resources, Error>, u8>")]
pub struct GetResource {
    pub method: Method,
    pub path: String,
}

impl Handler<CreateOrUpdateResource> for DbActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: CreateOrUpdateResource, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Resources>("Resources");
        Box::pin(async move {
            match collection
                .find_one(doc! {"path" : &msg.path, "method" : &msg.method}, None)
                .await
            {
                Ok(Some(_)) => {
                    match collection
                        .update_one(
                            doc! {
                                "method" : &msg.method,
                                "path" : &msg.path
                            },
                            doc! {
                                "$set" : {"access" : &msg.access}
                            },
                            None,
                        )
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            println!("e : {:?}", e);
                            Err(())
                        }
                    }
                }
                Ok(None) => match collection.insert_one(Resources::new(&msg), None).await {
                    Ok(_) => Ok(()),
                    _ => Err(()),
                },
                _ => Err(()),
            }
        })
    }
}

impl Handler<GetResource> for DbActor {
    type Result = ResponseFuture<Result<Result<Resources, Error>, u8>>;

    fn handle(&mut self, msg: GetResource, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Resources>("Resources");
        Box::pin(async move {
            match collection
                .find_one(doc! {"path" : &msg.path, "method" : &msg.method}, None)
                .await
            {
                Ok(Some(res)) => Ok(Ok(res)),
                Ok(None) => Err(0),
                _ => Err(1),
            }
        })
    }
}
