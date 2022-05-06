use crate::actors::cache::tokens::BanUser;
use crate::actors::database::users::DeleteUser;
use crate::state::AppState;
use actix_web::{delete, web::Data, HttpMessage, HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde_json::json;

#[delete("")]
pub async fn delete(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let id_str = req.extensions().get::<String>().unwrap().to_owned();
    let id = ObjectId::parse_str(id_str.to_owned()).unwrap();
    let db = state.as_ref().db.clone();
    let cache = state.as_ref().cache.clone();
    match cache
        .send(BanUser {
            id: id_str,
            dur: 24 * 60 * 60,
        })
        .await
    {
        Ok(Ok(_)) => {}
        _ => {
            return HttpResponse::InternalServerError().json(json!({
                "issues" : ["something went wrong"]
            }));
        }
    }
    match db.send(DeleteUser { _id: id }).await {
        Ok(Ok(Ok(_))) => HttpResponse::Ok().json(json!({
            "message": "Your account was deleted successfully"
        })),
        Ok(Err(e)) => HttpResponse::BadRequest().json(json!({ "message": e })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
