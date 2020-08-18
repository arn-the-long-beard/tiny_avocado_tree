use crate::handlers::secret::create_secret_key;
use crate::models::error::ServiceError;
use crate::models::error::ServiceError::BadRequest;
use crate::models::user::FullUser;
use actix_web::{web, HttpResponse};
use arangors::document::options::InsertOptions;
use arangors::Connection;
use shared::models::power::Power;
use shared::models::user::User;
use std::sync::Arc;

/// Register a user on the db
pub async fn register_user(
    user_payload: web::Json<User>,
    connection: web::Data<Arc<Connection>>,
) -> Result<HttpResponse, ServiceError> {
    // todo make different db name for environment

    let user = validate_and_unwrap(user_payload);

    if user.is_err() {
        return Err(user.unwrap_err());
    }
    let user = user.unwrap();

    let database = connection
        .db("tiny_avocado_tree")
        .await
        .expect("Should load the db");

    let collection = database
        .collection("users")
        .await
        .expect("Should load the collection");

    // todo add global secret so to have control to validate or invalidate
    let secret = create_secret_key(connection, user.credentials.username().to_string()).await;

    if secret.is_err() {
        return Err(ServiceError::InternalServerError);
    }
    let full_user = FullUser::create_new_from_user_with_hash(user, secret.unwrap().as_str());

    let new_user = collection
        .create_document(full_user, InsertOptions::builder().return_new(true).build())
        .await;

    if new_user.is_ok() {
        let full_user_doc = new_user.unwrap();
        let full_user = full_user_doc.new_doc().unwrap();
        Ok(HttpResponse::Ok().json(full_user.map_to_info()))
    } else {
        Err(ServiceError::InternalServerError)
    }
}

/// Check the user input on the user object
fn validate_and_unwrap(user: web::Json<User>) -> Result<User, ServiceError> {
    if user.last_name.is_empty() {
        Err(BadRequest("Last name cannot be empty".to_string()))
    } else if user.first_name.is_empty() {
        Err(BadRequest("First name cannot be empty".to_string()))
    } else if user.credentials.username().is_empty() {
        // todo add check that the username does not already exist
        Err(BadRequest("Username cannot be empty".to_string()))
    } else if user.credentials.email().is_empty() {
        //todo add better validation for email
        //todo check that email is not already taken
        Err(BadRequest("Email cannot be empty".to_string()))
    } else if user.credentials.password().is_empty() {
        //todo add better validation for password as well
        Err(BadRequest("Password cannot be empty".to_string()))
    } else if Power::calculate_power(user.credentials.password().to_string()) < 101 {
        Err(BadRequest("Password is too wek".to_string()))
    } else {
        Ok(user.into_inner())
    }
}
