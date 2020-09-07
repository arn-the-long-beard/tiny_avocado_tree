use crate::utils::password::hash_password;
use serde::{Deserialize, Serialize};
use shared::models::{
    auth::AuthData,
    user::{LoggedUser, User},
};

/// Full user model exclusive to back end
#[derive(Serialize, Deserialize)]
pub struct FullUser {
    pub first_name: String,
    pub last_name: String,
    hash: String,
    pub emails: Vec<String>,
    pub username: String,
}

impl FullUser {
    /// Map to a full User and use secret Key to create salty stuff
    pub fn create_new_from_user_with_hash(user: User, secret_key: &str) -> FullUser {
        let hash = hash_password(user.credentials.password(), secret_key).unwrap();

        FullUser {
            first_name: user.first_name,
            last_name: user.last_name,
            hash,
            emails: vec![user.credentials.email().to_string()],
            username: user.credentials.username().to_string(),
        }
    }

    /// Return only information
    pub fn map_to_info(&self) -> User {
        let mut info_cred = AuthData::default();
        info_cred.set_email((&self.emails.first().unwrap()).to_string());
        info_cred.set_username((&self.username).to_string());

        User {
            first_name: (&self.first_name).to_string(),
            last_name: (&self.last_name).to_string(),
            credentials: info_cred,
        }
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn to_logged_user(&self) -> LoggedUser {
        LoggedUser::new(
            (&self.first_name).to_string(),
            (&self.last_name).to_string(),
            (&self.username).to_string(),
            (&self.emails.first().unwrap()).to_string(),
        )
    }
}
