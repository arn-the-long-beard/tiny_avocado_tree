use crate::models::auth::AuthData;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

pub struct LoggedUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}
