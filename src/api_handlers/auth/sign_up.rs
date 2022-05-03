use crate::actors::database::users::CreateUser;
use crate::api_handlers::auth::models::users::UserDataSignUp;
use crate::api_handlers::auth::schema::get_sign_up_schema;
use crate::state::AppState;
use crate::utils::passwords::hash_password;
use crate::utils::validations::validate;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/sign_up")]
pub async fn sign_up(user: Json<UserDataSignUp>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let salt = &state.as_ref().salt;
    let salt = salt.clone();
    let user = user.into_inner();
    let user_json = match serde_json::to_value(&user) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };
    match validate(get_sign_up_schema(), user_json) {
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
            access_level: 1,
        })
        .await
    {
        Ok(Ok(Ok(_))) => HttpResponse::Ok().json(json!({
            "message": "Your account was created successfully, use login api to get auth token"
        })),
        Ok(Err(e)) => HttpResponse::BadRequest().json(json!({ "issues": [e], })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
