use crate::api_handlers::auth::models::users::UserDataSignOut;
use crate::api_handlers::auth::schema::get_sign_out_schema;
use crate::state::AppState;
use crate::utils::tokens::revoke_token;
use crate::utils::validations::validate;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/sign_out")]
pub async fn sign_out(token: Json<UserDataSignOut>, state: Data<AppState>) -> impl Responder {
    let cache = state.as_ref().cache.clone();
    let token = token.into_inner();
    let token_json = match serde_json::to_value(&token) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };

    match validate(get_sign_out_schema(), token_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    match revoke_token(token.refresh_token.unwrap(), cache.clone()).await {
        Ok(s) if s => HttpResponse::Ok().json(json!({
            "Message" : "Sing out was successful."
        })),
        Ok(s) if !s => HttpResponse::Unauthorized().json(json!({
            "Message" : "Invalid token."
        })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
