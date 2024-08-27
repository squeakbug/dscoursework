use std::fmt::{Debug, Display, Formatter};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_port: String,
    pub database_url: String,
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
        let listen_port = config_default("TICKET_SERVICE__LISTEN_PORT", "8080");
        let database_url = config("TICKET_SERVICE__DATABASE_URL")?;

        let config = Config {
            listen_port,
            database_url,
        };

        Ok(config)
    }
}
