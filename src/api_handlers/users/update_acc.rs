use crate::actors::database::users::UpdateUser;
use crate::api_handlers::users::models::users::UserDataUpdate;
use crate::api_handlers::users::schema::get_update_schema;
use crate::state::AppState;
use crate::utils::passwords::hash_password;
use crate::utils::validations::validate;
use actix_web::{
    put,
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use bson::oid::ObjectId;
use serde_json::json;

#[put("")]
pub async fn update(
    req: HttpRequest,
    user: Json<UserDataUpdate>,
    state: Data<AppState>,
) -> impl Responder {
    let id = ObjectId::parse_str(req.extensions().get::<String>().unwrap()).unwrap();
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
    match validate(get_update_schema(), user_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    
    //check if user has non None fields
    if !(user.email.is_some()
        || user.first_name.is_some()
        || user.last_name.is_some()
        || user.password.is_some())
    {
        return HttpResponse::BadRequest().json(json!({ "issues": ["At least one field must be provided"] }));
    }
    let hashed_password = if let Some(p) = user.password {
        match hash_password(salt, p) {
            Ok(val) => Some(val),
            _ => {
                return HttpResponse::Unauthorized().json(json!({
                    "issues" : ["something went wrong"]
                }));
            }
        }
    } else {
        None
    };
    match db
        .send(UpdateUser {
            _id: id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: hashed_password,
        })
        .await
    {
        Ok(Ok(Ok(_))) => HttpResponse::Ok().json(json!({
            "message": "Your account was updated successfully"
        })),
        Ok(Err(e)) => HttpResponse::BadRequest().json(json!({ "message": e })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
