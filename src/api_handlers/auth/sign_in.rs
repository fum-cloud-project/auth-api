use crate::actors::database::users::GetUser;
use crate::api_handlers::auth::models::users::UserDataSignIn;
use crate::api_handlers::auth::schema::get_sign_in_schema;
use crate::state::AppState;
use crate::utils::passwords::verify_password;
use crate::utils::tokens::gen_tokens;
use crate::utils::validations::validate;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/sign_in")]
pub async fn sign_in(user: Json<UserDataSignIn>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let secret = &state.as_ref().secret;
    let secret = secret.clone();
    let user = user.into_inner();
    let user_json = match serde_json::to_value(&user) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };
    match validate(get_sign_in_schema(), user_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    match db
        .send(GetUser {
            email: user.email.unwrap(),
        })
        .await
    {
        Ok(Ok(Ok(res))) => {
            let user_id = res._id;
            let user_password = res.password;
            let user_access_level = res.access_level;
            if verify_password(user_password, user.password.unwrap()) {
                match gen_tokens(
                    user_id.to_hex(),
                    user_access_level,
                    state.as_ref().cache.clone(),
                    secret,
                )
                .await
                {
                    Ok((acc_tok, ref_tok)) => HttpResponse::Ok()
                        .json(json!({ "access_token": acc_tok, "refresh_token": ref_tok })),
                    _ => HttpResponse::InternalServerError().json(json!({
                        "issues" : ["something went wrong"]
                    })),
                }
            } else {
                HttpResponse::Unauthorized().json(json!({
                    "issues" : ["wrong password or email"],
                }))
            }
        }
        Ok(Err(e)) if e == 0 => HttpResponse::Unauthorized().json(json!({
            "issues" : ["wrong password or email"]
        })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
