use crate::actors::database::users::UpdateUserAdmin;
use crate::api_handlers::users::models::users::UserDataUpdateAdmin;
use crate::api_handlers::users::schema::get_admin_update_schema;
use crate::state::AppState;
use crate::utils::passwords::hash_password;
use crate::utils::validations::validate;
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use bson::oid::ObjectId;
use serde_json::json;

#[put("/{id}")]
pub async fn update_admin(
    id: Path<(String,)>,
    user: Json<UserDataUpdateAdmin>,
    state: Data<AppState>,
) -> impl Responder {
    let id = match ObjectId::parse_str(id.into_inner().0) {
        Ok(v) => v,
        _ => {
            return HttpResponse::BadRequest().json(json!({ "issues": ["Bad user ID"] }));
        }
    };
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
    match validate(get_admin_update_schema(), user_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
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
        .send(UpdateUserAdmin {
            _id: id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: hashed_password,
            access_level: user.access_level,
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
