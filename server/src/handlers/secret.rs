use crate::models::{error::ServiceError, roots::Roots};
use actix_web::web;
use arangors::{document::options::InsertOptions, ClientError, Connection};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{collections::HashMap, iter, sync::Arc};

/// Create a secret key on an other db
pub async fn create_secret_key(
    connection: web::Data<Arc<Connection>>,
    username: String,
) -> Result<String, ServiceError> {
    let database = connection
        .db("avocado_trunk")
        .await
        .expect("Should load the collection");
    let collection = database
        .collection("roots")
        .await
        .expect("Should load the collection");
    let secret = generate_key();
    let roots = Roots::new(secret.clone(), username);
    let new_key = collection
        .create_document(roots, InsertOptions::builder().silent(true).build())
        .await;

    if new_key.is_ok() {
        Ok(secret)
    } else {
        Err(ServiceError::InternalServerError)
    }
}
fn generate_key() -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(45)
        .collect();

    chars.to_string()
}

pub async fn read_secret_key(
    connection: web::Data<Arc<Connection>>,
    username: String,
) -> Result<String, ServiceError> {
    let database = connection.db("avocado_trunk").await.unwrap();

    let mut map = HashMap::new();
    map.insert("username", serde_json::to_value(username).unwrap());
    let res: Result<Vec<String>, ClientError> = database
        .aql_bind_vars(
            "FOR r in roots FILTER  r.username == @username return r.main",
            map,
        )
        .await;

    if res.is_ok() {
        //todo add a check if many maybe ?
        let mut vec = res.unwrap();

        let secret = vec.pop();

        if secret.is_none() {
            Err(ServiceError::BadRequest(
                "Your credentials are wrong".to_string(),
            ))
        } else {
            Ok(secret.unwrap())
        }
    } else {
        let err = res.unwrap_err();
        eprintln!("Error happened :{:?}", err);
        Err(ServiceError::InternalServerError)
    }
}
