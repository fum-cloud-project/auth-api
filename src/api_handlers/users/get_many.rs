use crate::actors::database::users::{CompareType, CountUsersWithFilter, GetUsersWithFilter};
use crate::state::AppState;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct UserFilter {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub access_level: Option<i32>,
    pub access_level_cmp: Option<u8>,
}

#[get("")]
pub async fn get_many(filter: Query<UserFilter>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let page = filter.page.unwrap_or(1);
    let size = filter.size.unwrap_or(20);
    let acc_cmp = if let Some(acc) = filter.access_level {
        let acc_c = match filter.access_level_cmp.unwrap_or(0) {
            1 => CompareType::GT,
            2 => CompareType::GTE,
            3 => CompareType::LT,
            4 => CompareType::LTE,
            _ => CompareType::EQ,
        };
        Some((acc, acc_c))
    } else {
        None
    };
    let skip = (page - 1) * size;
    let count = match db
        .send(CountUsersWithFilter {
            first_name: filter.first_name.clone(),
            last_name: filter.last_name.clone(),
            email: filter.email.clone(),
            access_level: acc_cmp,
        })
        .await
    {
        Ok(Ok(c)) => c,
        _ => {
            return HttpResponse::InternalServerError().json(json!({
                    "issues" : ["something went wrong"],
            }));
        }
    };
    match db
        .send(GetUsersWithFilter {
            limit: size as i64,
            skip: skip,
            first_name: filter.first_name.clone(),
            last_name: filter.last_name.clone(),
            email: filter.email.clone(),
            access_level: acc_cmp,
        })
        .await
    {
        Ok(Ok(s)) => HttpResponse::Ok().json(json!({
            "message": "Users retrieved successfully",
            "data" : {
                "page" : page,
                "total_count" : count,
                "data" : s
            },
        })),
        Ok(Err(e)) => HttpResponse::BadRequest().json(json!({ "message": e })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
