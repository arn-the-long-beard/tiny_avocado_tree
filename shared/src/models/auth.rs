use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
/// Base Credential used for user authentication
pub struct AuthData {
    pub email: String,
    pub username: String,
    password: String,
}
/// Setters and getters for password
impl AuthData {
    pub fn set_password(&mut self, pwd: String) {
        self.password = pwd
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }
}
