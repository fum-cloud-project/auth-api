use crate::actix::Addr;
use crate::actors::database::resources::CreateOrUpdateResource;
use crate::actors::database::users::CreateUser;
use crate::actors::database::DbActor;
use crate::db_schemas::resources::Method;
use crate::utils::passwords::hash_password;
use log::{error, info};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct Resource {
    path: String,
    access: i32,
    method: String,
}

fn read_resources_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Resource>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let r = serde_json::from_reader(reader)?;

    Ok(r)
}

pub async fn add_resources(
    db: Addr<DbActor>,
    salt: Arc<&str>,
    path: &Path,
    admin_email: &str,
    admin_password: &str,
) -> Result<(), ()> {
    let db = db.clone();
    let resources = match read_resources_from_file(path) {
        Ok(s) => s,
        Err(_) => {
            return Err(());
        }
    };

    for resource in resources {
        let method = match resource.method.clone().as_str() {
            "POST" => Method::POST,
            "GET" => Method::GET,
            "DELETE" => Method::DELETE,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "TRACE" => Method::TRACE,
            "HEAD" => Method::HEAD,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::INVALID,
        };
        match db
            .send(CreateOrUpdateResource {
                path: resource.path.clone(),
                access: resource.access,
                method: method,
            })
            .await
        {
            Ok(Ok(_)) => {
                info!(
                    "Added {} {} {}",
                    resource.method, resource.path, resource.access
                );
            }
            _ => {
                error!("Adding to DB failed!");
                return Err(());
            }
        }
    }

    let hashed_password = hash_password(salt, admin_password.to_owned()).unwrap();
    match db
        .send(CreateUser {
            first_name: "admin".to_owned(),
            last_name: "admin".to_owned(),
            email: admin_email.to_owned(),
            password: hashed_password,
            access_level: 1000,
        })
        .await
    {
        Ok(Ok(Ok(_))) => {}
        Ok(Err(_)) => {}
        _ => {
            return Err(());
        }
    }
    Ok(())
}
