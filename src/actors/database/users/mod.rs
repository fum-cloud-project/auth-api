use crate::actix::{Handler, Message, ResponseFuture};
use crate::actors::database::DbActor;
use crate::db_schemas::users::Users;
use bson::oid::ObjectId;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{bson::doc, options::FindOptions};
#[derive(Message)]
#[rtype(result = "Result<Result<InsertOneResult, Error>, String>")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub access_level: u16,
}

#[derive(Message)]
#[rtype(result = "Result<Users, Error>")]
pub struct UpdateUser_ {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub access_level: Option<u16>,
    pub password: Option<String>,
}

#[derive(Message)]
#[rtype(result = "Result<Users, Error>")]
pub struct DeleteUser {
    pub _id: ObjectId,
}

impl Handler<CreateUser> for DbActor {
    type Result = ResponseFuture<Result<Result<InsertOneResult, Error>, String>>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        println!("handling");
        let collection = self.0.collection::<Users>("Users");
        println!("handling");
        Box::pin(async move {
            match collection.find_one(doc! {"email" : &msg.email}, None).await {
                Ok(_) => Err(format!("User already exists.")),
                _ => Ok(collection.insert_one(Users::new(msg), None).await),
            }
        })
    }
}

// impl Handler<CreateUser> for DbActor {
//     type Result = QueryResult<User>;

//     fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");
//         let new_user = NewUser {
//             first_name: msg.first_name,
//             last_name: msg.last_name,
//             email: msg.email,
//             access_level: msg.access_level,
//             password: msg.password,
//             created_at: msg.created_at,
//         };

//         diesel::insert_into(users)
//             .values(new_user)
//             .get_result::<User>(&conn)
//     }
// }

// impl Handler<UpdateUser_> for DbActor {
//     type Result = QueryResult<User>;

//     fn handle(&mut self, msg: UpdateUser_, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");
//         let handle = diesel::update(users).filter(id.eq(msg.id));
//         handle
//             .set(&UpdateUser {
//                 first_name: msg.first_name,
//                 last_name: msg.last_name,
//                 email: msg.email,
//                 access_level: msg.access_level,
//                 password: msg.password,
//                 is_deleted: None,
//             })
//             .get_result::<User>(&conn)
//     }
// }

// impl Handler<DeleteUser> for DbActor {
//     type Result = QueryResult<User>;

//     fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");

//         let handle = diesel::update(users).filter(id.eq(msg.id));
//         handle
//             .set(&UpdateUser {
//                 first_name: None,
//                 last_name: None,
//                 email: None,
//                 access_level: None,
//                 password: None,
//                 is_deleted: Some(true),
//             })
//             .get_result::<User>(&conn)
//     }
// }

// impl Handler<GetUsers> for DbActor {
//     type Result = QueryResult<Vec<User>>;

//     fn handle(&mut self, _msg: GetUsers, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");
//         users.get_results::<User>(&conn)
//     }
// }

// impl Handler<GetUser> for DbActor {
//     type Result = QueryResult<User>;

//     fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");
//         users.filter(email.eq(msg.email)).get_result::<User>(&conn)
//     }
// }
