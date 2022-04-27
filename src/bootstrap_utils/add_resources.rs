use crate::actix::Addr;
use crate::actors::database::resources::CreateOrUpdateResource;
use crate::actors::database::DbActor;
use crate::db_schemas::resources::{Method, Resources};

pub async fn add_resources(db: Addr<DbActor>) -> Result<(), ()> {
    let db = db.clone();

    match db
        .send(CreateOrUpdateResource {
            path: format!("/api/auth/sign_in"),
            access: 0,
            method: Method::POST,
        })
        .await
    {
        Ok(Ok(_)) => {
            println!("Added POST /api/auth/sign_in")
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateResource {
            path: format!("/api/auth/sign_up"),
            access: 0,
            method: Method::POST,
        })
        .await
    {
        Ok(Ok(_)) => {
            println!("Added POST /api/auth/sign_up")
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateResource {
            path: format!("/api/auth/sign_out"),
            access: 1,
            method: Method::POST,
        })
        .await
    {
        Ok(Ok(_)) => {
            println!("Added POST /api/auth/sign_out")
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    Ok(())
}
