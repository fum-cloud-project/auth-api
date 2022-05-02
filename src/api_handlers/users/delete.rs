use crate::actors::cache::tokens::BanUser;
use crate::actors::database::users::DeleteUser;
use crate::state::AppState;
use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse, Responder,
};
use bson::oid::ObjectId;
use serde_json::json;

#[delete("/{id}")]
pub async fn delete_admin(id: Path<(String,)>, state: Data<AppState>) -> impl Responder {
    let id_str = id.into_inner().0;
    let id = match ObjectId::parse_str(id_str.clone()) {
        Ok(v) => v,
        _ => {
            return HttpResponse::BadRequest().json(json!({ "issues": ["Bad user ID"] }));
        }
    };
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
            "message": "User was deleted successfully"
        })),
        Ok(Err(e)) => HttpResponse::NotFound().json(json!({ "issues": [e], })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
