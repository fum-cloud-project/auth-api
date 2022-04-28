use crate::actix::{Handler, Message, ResponseFuture};
use crate::actors::database::DbActor;
use crate::db_schemas::users::Users;
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::{InsertOneResult, UpdateResult};
#[derive(Message)]
#[rtype(result = "Result<Result<InsertOneResult, Error>, String>")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub access_level: i32,
}

#[derive(Message)]
#[rtype(result = "Result<Result<UpdateResult, Error>, String>")]
pub struct UpdateUser {
    pub _id: ObjectId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Message)]
#[rtype(result = "Result<Result<UpdateResult, Error>, String>")]
pub struct PromoteUser {
    pub _id: ObjectId,
    pub access_level: Option<i32>,
}

#[derive(Message)]
#[rtype(result = "Result<Result<UpdateResult, Error>, String>")]
pub struct DeleteUser {
    pub _id: ObjectId,
}

#[derive(Message)]
#[rtype(result = "Result<Result<Users, Error>, u8>")]
pub struct GetUser {
    pub email: String,
}

impl Handler<CreateUser> for DbActor {
    type Result = ResponseFuture<Result<Result<InsertOneResult, Error>, String>>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection.find_one(doc! {"email" : &msg.email}, None).await {
                Ok(Some(_)) => Err(format!("User already exists.")),
                _ => Ok(collection.insert_one(Users::new(msg), None).await),
            }
        })
    }
}

impl Handler<UpdateUser> for DbActor {
    type Result = ResponseFuture<Result<Result<UpdateResult, Error>, String>>;

    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection.find_one(doc! {"_id" : &msg._id}, None).await {
                Ok(Some(user)) => {
                    if let Some(_) = &msg.email {
                        match collection.find_one(doc! {"email" : &msg.email}, None).await {
                            Ok(Some(_)) => {
                                return Err(format!("Email already exists."));
                            }
                            _ => {}
                        }
                    }
                    Ok(collection
                        .update_one(
                            doc! {"_id" : &msg._id},
                            doc! {
                                "$set" : {
                                    "first_name" : if let Some(first_name) = msg.first_name {first_name} else {user.first_name},
                                    "last_name" : if let Some(last_name) = msg.last_name {last_name} else {user.last_name},
                                    "email" : if let Some(email) = msg.email {email} else {user.email},
                                    "password" : if let Some(password) = msg.password {password} else {user.password},
                                }
                            },
                            None)
                        .await)
                }
                _ => Err(format!("User does not exist.")),
            }
        })
    }
}

impl Handler<DeleteUser> for DbActor {
    type Result = ResponseFuture<Result<Result<UpdateResult, Error>, String>>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection.find_one(doc! {"_id" : &msg._id}, None).await {
                Ok(Some(_)) => Ok(collection
                    .update_one(
                        doc! {
                            "_id" : &msg._id
                        },
                        doc! {
                            "$set" : {"is_deleted" : true}
                        },
                        None,
                    )
                    .await),
                _ => Err(format!("User does not exist.")),
            }
        })
    }
}

impl Handler<GetUser> for DbActor {
    type Result = ResponseFuture<Result<Result<Users, Error>, u8>>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection.find_one(doc! {"email" : &msg.email}, None).await {
                Ok(Some(u)) => Ok(Ok(u)),
                Ok(None) => Err(0),
                _ => Err(1),
            }
        })
    }
}
