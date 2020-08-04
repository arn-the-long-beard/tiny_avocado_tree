use crate::models::error::ServiceError;
use arangors::Connection;
use env_logger::Env;
use openssl::error::ErrorStack;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

/// Struct to init variable for config for server
#[derive(Default)]
pub struct Init {
    domain: String,
    ssl_private_key: String,
    ssl_public_certificate: String,
    db_url: String,
    db_admin_user: String,
    db_password_user: String,
}
/// Init fails if one fails
impl Init {
    /// Instantiate init with value from .env file
    pub fn new() -> Self {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
        dotenv::from_filename("../config/.env").ok();
        std::env::set_var(
            "RUST_LOG",
            "tiny_avocado_tree_server=debug,actix_web=info,actix_server=info",
        );
        Init {
            domain: std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
            ssl_private_key: std::env::var("PRIVATE_KEY_PATH").unwrap(),
            ssl_public_certificate: std::env::var("PUBLIC_CERTIFICATE_PATH").unwrap(),
            db_url: std::env::var("DB_URL").unwrap(),
            db_admin_user: std::env::var("DB_ADMIN").unwrap(),
            db_password_user: std::env::var("DB_PASSWORD").unwrap(),
        }
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }
    pub fn build_ssl(&self) -> SslAcceptorBuilder {
        let builder_res = SslAcceptor::mozilla_intermediate(SslMethod::tls());

        if builder_res.is_err() {
            panic!("Error during ssl init")
        }
        let mut builder = builder_res.unwrap();

        builder
            .set_private_key_file(&self.ssl_private_key, SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(&self.ssl_public_certificate)
            .unwrap();

        builder
    }

    pub async fn connect_db(&self) -> Connection {
        Connection::establish_jwt(&self.db_url, &self.db_admin_user, &self.db_password_user)
            .await
            .unwrap()
    }
}
