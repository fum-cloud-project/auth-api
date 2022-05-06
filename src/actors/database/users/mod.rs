use crate::actix::{Handler, Message, ResponseFuture};
use crate::actors::database::DbActor;
use crate::db_schemas::users::{PasswordlessUsers, Users};
use bson::oid::ObjectId;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::{InsertOneResult, UpdateResult};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CompareType {
    EQ,
    LTE,
    GTE,
    LT,
    GT,
}

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
pub struct UpdateUserAdmin {
    pub _id: ObjectId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
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

#[derive(Message)]
#[rtype(result = "Result<Result<Users, Error>, u8>")]
pub struct GetUserWithId {
    pub _id: ObjectId,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<PasswordlessUsers>, u8>")]
pub struct GetUsersWithFilter {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub access_level: Option<(i32, CompareType)>,
    pub skip: u64,
    pub limit: i64,
}

#[derive(Message)]
#[rtype(result = "Result<u64, u8>")]
pub struct CountUsersWithFilter {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub access_level: Option<(i32, CompareType)>,
}

impl Handler<CreateUser> for DbActor {
    type Result = ResponseFuture<Result<Result<InsertOneResult, Error>, String>>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection
                .find_one(doc! {"email" : &msg.email, "is_deleted" : false}, None)
                .await
            {
                Ok(Some(_)) => Err(format!("User already exists.")),
                _ => Ok(collection.insert_one(Users::new(msg), None).await),
            }
        })
    }
}

impl Handler<UpdateUserAdmin> for DbActor {
    type Result = ResponseFuture<Result<Result<UpdateResult, Error>, String>>;

    fn handle(&mut self, msg: UpdateUserAdmin, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection.find_one(doc! {"_id" : &msg._id}, None).await {
                Ok(Some(user)) => {
                    if let Some(_) = &msg.email {
                        match collection
                            .find_one(doc! {"email" : &msg.email, "is_deleted" : false}, None)
                            .await
                        {
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
                                    "access_level" : if let Some(access_level) = msg.access_level { access_level } else {user.access_level},
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

impl Handler<UpdateUser> for DbActor {
    type Result = ResponseFuture<Result<Result<UpdateResult, Error>, String>>;

    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection.find_one(doc! {"_id" : &msg._id}, None).await {
                Ok(Some(user)) => {
                    if let Some(_) = &msg.email {
                        match collection
                            .find_one(doc! {"email" : &msg.email, "is_deleted" : false}, None)
                            .await
                        {
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
            match collection
                .find_one(doc! {"_id" : &msg._id, "is_deleted" : false}, None)
                .await
            {
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
            match collection
                .find_one(doc! {"email" : &msg.email, "is_deleted" : false}, None)
                .await
            {
                Ok(Some(u)) => Ok(Ok(u)),
                Ok(None) => Err(0),
                _ => Err(1),
            }
        })
    }
}

impl Handler<GetUserWithId> for DbActor {
    type Result = ResponseFuture<Result<Result<Users, Error>, u8>>;

    fn handle(&mut self, msg: GetUserWithId, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            match collection
                .find_one(doc! {"_id" : &msg._id, "is_deleted" : false}, None)
                .await
            {
                Ok(Some(u)) => Ok(Ok(u)),
                Ok(None) => Err(0),
                _ => Err(1),
            }
        })
    }
}

impl Handler<GetUsersWithFilter> for DbActor {
    type Result = ResponseFuture<Result<Vec<PasswordlessUsers>, u8>>;

    fn handle(&mut self, msg: GetUsersWithFilter, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<PasswordlessUsers>("Users");
        Box::pin(async move {
            let mut res: Vec<PasswordlessUsers> = Vec::new();
            let mut doc_find = doc! {"is_deleted" : false};
            let mut options = mongodb::options::FindOptions::default();
            options.limit = Some(msg.limit);
            options.skip = Some(msg.skip);
            options.projection = Some(doc! {
                "password" : 0
            });
            if let Some(first_name) = msg.first_name {
                doc_find.insert(
                    "first_name",
                    doc! {"$regex" : format!(".*{}.*", first_name).as_str()},
                );
            };
            if let Some(last_name) = msg.last_name {
                doc_find.insert(
                    "last_name",
                    doc! {"$regex" : format!(".*{}.*", last_name).as_str()},
                );
            };
            if let Some(email) = msg.email {
                doc_find.insert("email", doc! {"$regex" : format!(".*{}.*", email).as_str()});
            };
            if let Some((access_level, cmp)) = msg.access_level {
                doc_find.insert(
                    "access_level",
                    match cmp {
                        CompareType::EQ => {
                            doc! {"$eq" : access_level}
                        }
                        CompareType::GT => {
                            doc! {"$gt" : access_level}
                        }
                        CompareType::LT => {
                            doc! {"$lt" : access_level}
                        }
                        CompareType::GTE => {
                            doc! {"$gte" : access_level}
                        }
                        CompareType::LTE => {
                            doc! {"$lte" : access_level}
                        }
                    },
                );
            };
            let mut cur = match collection.find(doc_find, Some(options)).await {
                Ok(c) => c,
                _ => {
                    return Err(1);
                }
            };
            loop {
                match cur.try_next().await {
                    Ok(Some(u)) => {
                        res.push(u);
                    }
                    Ok(None) => {
                        return Ok(res);
                    }
                    _ => {
                        return Err(1);
                    }
                }
            }
        })
    }
}

impl Handler<CountUsersWithFilter> for DbActor {
    type Result = ResponseFuture<Result<u64, u8>>;

    fn handle(&mut self, msg: CountUsersWithFilter, _: &mut Self::Context) -> Self::Result {
        let collection = self.0.collection::<Users>("Users");
        Box::pin(async move {
            let mut doc_find = doc! {"is_deleted" : false};
            if let Some(first_name) = msg.first_name {
                doc_find.insert(
                    "first_name",
                    doc! {"$regex" : format!(".*{}.*", first_name).as_str()},
                );
            };
            if let Some(last_name) = msg.last_name {
                doc_find.insert(
                    "last_name",
                    doc! {"$regex" : format!(".*{}.*", last_name).as_str()},
                );
            };
            if let Some(email) = msg.email {
                doc_find.insert("email", doc! {"$regex" : format!(".*{}.*", email).as_str()});
            };
            if let Some((access_level, cmp)) = msg.access_level {
                doc_find.insert(
                    "access_level",
                    match cmp {
                        CompareType::EQ => {
                            doc! {"$eq" : access_level}
                        }
                        CompareType::GT => {
                            doc! {"$gt" : access_level}
                        }
                        CompareType::LT => {
                            doc! {"$lt" : access_level}
                        }
                        CompareType::GTE => {
                            doc! {"$gte" : access_level}
                        }
                        CompareType::LTE => {
                            doc! {"$lte" : access_level}
                        }
                    },
                );
            };
            match collection.count_documents(doc_find, None).await {
                Ok(c) => {
                    return Ok(c);
                }
                _ => {
                    return Err(1);
                }
            }
        })
    }
}
