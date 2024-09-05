use std::env;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_port: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub kafka_bootstrap_servers: String,
}

#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[allow(unused)]
fn config(name: &str) -> Result<String, ConfigError> {
    match env::var(name) {
        Err(_) => dotenv::var(name).map_err(|_| ConfigError {
            message: format!("{} must be set", name),
        }),
        Ok(res) => Ok(res),
    }
}

#[allow(unused)]
fn config_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or(dotenv::var(name).unwrap_or(default.into()))
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
        let listen_port = config("BONUS_SERVICE__LISTEN_PORT")?;
        let database_url = config("BONUS_SERVICE__DATABASE_URL")?;
        let jwt_secret = config("IDENTITY_SECRET_KEY")?;
        let kafka_bootstrap_servers = config("KAFKA__BOOTSTRAP_SERVERS")?;

        let config = Config {
            listen_port,
            database_url,
            jwt_secret,
            kafka_bootstrap_servers,
        };

        Ok(config)
    }
}
