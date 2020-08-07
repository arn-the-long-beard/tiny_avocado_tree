use serde::{Deserialize, Serialize};

/// Represent secret for user to hash their password
#[derive(Serialize, Deserialize)]
pub struct Roots {
    /// Main Secret Key for hashing password
    main: String,
    /// The user owning this secret Key
    username: String,
    /// Should be ISO date
    pub created_at: String,
}

impl Roots {
    pub fn new(main: String, username: String) -> Self {
        let created_at = chrono::Utc::now().to_string();

        Roots {
            main,
            username,
            created_at,
        }
    }
    pub fn main(&self) -> &str {
        &self.main
    }
    pub fn username(&self) -> &str {
        &self.username
    }
}
