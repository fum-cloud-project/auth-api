use crate::actors::database::users::CreateUser;
use crate::api_handlers::users::models::users::UserDataCreate;
use crate::api_handlers::users::schema::get_create_schema;
use crate::state::AppState;
use crate::utils::passwords::hash_password;
use crate::utils::validations::validate;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/")]
pub async fn create(user: Json<UserDataCreate>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let salt = &state.as_ref().salt;
    let salt = salt.to_string();
    let user = user.into_inner();
    let user_json = match serde_json::to_value(&user) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };
    match validate(get_create_schema(), user_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    let hashed_password = match hash_password(salt, user.password.unwrap()) {
        Ok(val) => val,
        _ => {
            return HttpResponse::Unauthorized().json(json!({
                "issues" : ["something went wrong"]
            }));
        }
    };
    match db
        .send(CreateUser {
            first_name: user.first_name.unwrap(),
            last_name: user.last_name.unwrap(),
            email: user.email.unwrap(),
            password: hashed_password,
            access_level: user.access_level.unwrap(),
        })
        .await
    {
        Ok(Ok(Ok(_))) => HttpResponse::Ok().json(json!({
            "message": "User was created successfully"
        })),
        Ok(Err(e)) => HttpResponse::BadRequest().json(json!({ "message": e })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
