use arangors::Connection;
use env_logger::Env;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

/// Struct to init variable for config for server
#[derive(Default)]
pub struct Init {
    domain: String,
    ssl_private_key: String,
    ssl_public_certificate: String,
    db_url: String,
    db_admin_user: String,
    db_password_user: String,
    workers: usize,
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
            workers: std::env::var("WORKERS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap(),
        }
    }
    pub fn workers(&self) -> &usize {
        &self.workers
    }
    pub fn domain(&self) -> &str {
        &self.domain
    }
    pub fn build_ssl_config(&self) -> ServerConfig {
        // todo this looks like good stuff but need to spend more time to make better config
        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open(&self.ssl_public_certificate).unwrap());
        let key_file = &mut BufReader::new(File::open(&self.ssl_private_key).unwrap());

        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();

        let result = config.set_single_cert(cert_chain, keys.remove(0));

        if result.is_err() {
            panic!(result.unwrap_err());
        }

        config
    }

    pub async fn connect_db(&self) -> Connection {
        Connection::establish_jwt(&self.db_url, &self.db_admin_user, &self.db_password_user)
            .await
            .unwrap()
    }
}
