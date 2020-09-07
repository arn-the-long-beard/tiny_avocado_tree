use crate::models::error::ServiceError;
use actix_web::{web, HttpResponse, ResponseError};

use crate::{handlers::secret::read_secret_key, models::user::FullUser, utils::password::verify};
use actix_identity::Identity;
use arangors::{ClientError, Connection};
use shared::models::{auth::LoginCredentials, user::User};
use std::{collections::HashMap, sync::Arc};

pub async fn login(
    auth_data: web::Json<LoginCredentials>,
    connection: web::Data<Arc<Connection>>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    let database = connection
        .db("tiny_avocado_tree")
        .await
        .expect("Should load the collection");
    let collection = database
        .collection("users")
        .await
        .expect("Should load the collection");

    let mut map = HashMap::new();
    map.insert(
        "username",
        serde_json::to_value(auth_data.target()).unwrap(),
    );
    let user_response: Result<Vec<FullUser>, ClientError> = collection
        .db()
        .aql_bind_vars(
            "FOR r in users FILTER  r.username == @username return r",
            map,
        )
        .await;

    let secret = read_secret_key(connection, auth_data.target().to_string()).await;

    if user_response.is_ok() && secret.is_ok() {
        let mut users = user_response.unwrap();

        match users.len() {
            0 => Err(ServiceError::BadRequest(
                "Your credentials are wrong".to_string(),
            )),
            1 => {
                let user = users.pop().unwrap();
                let check = verify(user.hash(), auth_data.password(), secret.unwrap().as_str());

                if check.is_err() {
                    Err(ServiceError::BadRequest(
                        "Your credentials are wrong".to_string(),
                    ))
                } else {
                    let valid = check.unwrap();
                    if valid {
                        id.remember((&user.username).to_string());
                        Ok(HttpResponse::Ok().json(user.to_logged_user()))
                    } else {
                        Err(ServiceError::BadRequest(
                            "Your credentials are wrong".to_string(),
                        ))
                    }
                }
            }
            _ => {
                eprintln!("It should not be more than one result");
                Err(ServiceError::InternalServerError)
            }
        }
    } else {
        Err(ServiceError::BadRequest(
            "Your credentials are wrong".to_string(),
        ))
    }
}
