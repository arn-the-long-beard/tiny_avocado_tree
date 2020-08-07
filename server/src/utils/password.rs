use argonautica::{Hasher, Verifier};

use crate::models::error::ServiceError;

// todo make code for production because of warning bellow
// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash_password(password: &str, secret: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(secret)
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
}

pub fn verify(hash: &str, password: &str, secret: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret)
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}
