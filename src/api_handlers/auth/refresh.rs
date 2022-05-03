use crate::api_handlers::auth::models::users::UserDataRefresh;
use crate::api_handlers::auth::schema::get_refresh_schema;
use crate::state::AppState;
use crate::utils::tokens::refresh_token;
use crate::utils::validations::validate;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/refresh")]
pub async fn refresh(token: Json<UserDataRefresh>, state: Data<AppState>) -> impl Responder {
    let cache = state.as_ref().cache.clone();
    let secret = &state.as_ref().secret;
    let secret = secret.clone();
    let token = token.into_inner();
    let token_json = match serde_json::to_value(&token) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };

    match validate(get_refresh_schema(), token_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    match refresh_token(token.refresh_token.unwrap(), secret, cache.clone()).await {
        Ok((acc_tok, ref_tok)) => {
            HttpResponse::Ok().json(json!({ "access_token": acc_tok, "refresh_token": ref_tok }))
        }
        Err(2) => HttpResponse::Unauthorized().json(json!({
            "Message" : "Access token is still valid."
        })),
        Err(0) => HttpResponse::Unauthorized().json(json!({
            "Message" : "Invalid token."
        })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
