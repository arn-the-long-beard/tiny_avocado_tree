use arangors::Connection;
use env_logger::Env;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
/// Struct to init variable for config for server
#[derive(Default, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Init {
    /// The domain/url of the server, by default is localhost
    #[serde(default = "default_domain")]
    domain: String,
    /// the path to the private_key, use linux path in /etc in production
    private_key_path: String,
    /// the path to the public_key, use linux path in /etc in production
    public_certificate_path: String,
    /// arangodb single main db url address
    db_url: String,
    /// arangodb single root admin user
    db_admin: String,
    /// arangodb admin password
    db_password: String,
    #[serde(default = "default_workers")]
    /// how many workers to run the web server with
    workers: usize,
    /// The different mode for the server -> dev, test, staging, production
    /// in dev & test, front end is compiled with debug mode active, same on backend of compiled code
    /// in test, same as dev but with production data
    /// in staging, we have production data, & production build
    /// in production,  the code is actually used by our customers
    mode: String,
}

fn default_domain() -> String {
    "localhost".to_string()
}

fn default_workers() -> usize {
    1
}

/// Init fails if one fails
impl Init {
    /// Instantiate init with value from .env file
    pub fn new() -> Self {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
        let file = File::open("../config/config.json").expect("Should load the config file");
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        serde_json::from_reader(reader).unwrap()
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
        let cert_file = &mut BufReader::new(
            File::open(&self.public_certificate_path).expect("should have open certificate"),
        );
        let key_file = &mut BufReader::new(
            File::open(&self.private_key_path).expect("should have open the key"),
        );

        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();

        let result = config.set_single_cert(cert_chain, keys.remove(0));

        if result.is_err() {
            panic!(result.unwrap_err());
        }

        config
    }

    pub async fn connect_db(&self) -> Connection {
        Connection::establish_jwt(&self.db_url, &self.db_admin, &self.db_password)
            .await
            .unwrap()
    }
}
