use crate::actors::database::users::DeleteUser;
use crate::state::AppState;
use actix_web::{delete, web::Data, HttpMessage, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde_json::json;

#[delete("/")]
pub async fn delete(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let id = ObjectId::parse_str(req.extensions().get::<String>().unwrap()).unwrap();
    let db = state.as_ref().db.clone();
    match db.send(DeleteUser { _id: id }).await {
        Ok(Ok(Ok(_))) => HttpResponse::Ok().json(json!({
            "message": "User was deleted successfully"
        })),
        Ok(Err(e)) => HttpResponse::BadRequest().json(json!({ "message": e })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
