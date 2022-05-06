use crate::actors::database::users::GetUserWithId;
use crate::state::AppState;
use actix_web::{get, web::Data, HttpMessage, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde_json::json;

#[get("/my_acc")]
pub async fn get_my_acc(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let id_str = req.extensions().get::<String>().unwrap().to_owned();
    let id = ObjectId::parse_str(id_str.to_owned()).unwrap();
    let db = state.as_ref().db.clone();
    match db.send(GetUserWithId { _id: id }).await {
        Ok(Ok(Ok(s))) => HttpResponse::Ok().json(json!({
            "message": "Your account was retrieved successfully",
            "data" : [
                {
                    "_id" : s._id,
                    "first_name" : s.first_name,
                    "last_name" : s.last_name,
                    "email" : s.email,
                    "access_level" : s.access_level
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
