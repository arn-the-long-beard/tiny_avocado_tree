use crate::models::auth::AuthData;

pub struct User {
    first_name: String,
    last_name: String,
    email: String,
}

pub struct LoggedUser {
    first_name: String,
    last_name: String,
    username: String,
}
