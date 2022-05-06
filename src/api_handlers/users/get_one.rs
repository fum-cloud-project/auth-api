use crate::actors::database::users::GetUserWithId;
use crate::state::AppState;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use bson::oid::ObjectId;
use serde_json::json;

#[get("/{id}")]
pub async fn get_one(id: Path<(String,)>, state: Data<AppState>) -> impl Responder {
    let id = match ObjectId::parse_str(id.into_inner().0) {
        Ok(v) => v,
        _ => {
            return HttpResponse::BadRequest().json(json!({ "issues": ["Bad user ID"] }));
        }
    };
    let db = state.as_ref().db.clone();
    match db.send(GetUserWithId { _id: id }).await {
        Ok(Ok(Ok(s))) => HttpResponse::Ok().json(json!({
            "message": "User retrieved successfully",
            "data" : [
                {
                    "_id" : s._id,
                    "first_name" : s.first_name,
                    "last_name" : s.last_name,
                    "email" : s.email,
                    "access_level" : s.access_level,
                    "creation_date" : s.creation_date,
                    "is_deleted" : s.is_deleted
                }
            ],
        })),
        Ok(Err(e)) if e == 0 => {
            HttpResponse::NotFound().json(json!({ "message": "This user does not exist" }))
        }
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
