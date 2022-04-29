use crate::actix::Addr;
use crate::actors::database::resources::CreateOrUpdateResource;
use crate::actors::database::DbActor;
use crate::db_schemas::resources::Method;
use log::{error, info};

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
            info!("Added POST /api/auth/sign_in")
        }
        _ => {
            error!("Adding to DB failed!");
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
            info!("Added POST /api/auth/sign_up")
        }
        _ => {
            error!("Adding to DB failed!");
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
            info!("Added POST /api/auth/sign_out")
        }
        _ => {
            error!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateResource {
            path: format!("/api/auth/refresh"),
            access: 0,
            method: Method::POST,
        })
        .await
    {
        Ok(Ok(_)) => {
            info!("Added POST /api/auth/refresh")
        }
        _ => {
            error!("Adding to DB failed!");
            return Err(());
        }
    }
    Ok(())
}
