use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_port: String,
    pub bonus_service_address: String,
    pub flight_service_address: String,
    pub ticket_service_address: String,

    pub rmq_address: String,

    pub jwt_secret: String,
}

use std::fmt::{Debug, Display, Formatter};

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
fn config_default(name: &str, default: impl Into<String>) -> String {
    env::var(name).unwrap_or(dotenv::var(name).unwrap_or(default.into()))
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
        let listen_port = config_default("GATEWAY_SERVICE__LISTEN_PORT", "8080");
        let bonus_service_address = config_default("BONUS_SERVICE_ADDRESS", "http://localhost:8050");
        let flight_service_address = config_default("FLIGHT_SERVICE_ADDRESS", "http://localhost:8060");
        let ticket_service_address = config_default("TICKET_SERVICE_ADDRESS", "http://localhost:8070");
        let rmq_address = config_default("RABBIT_MQ_ADDRESS", "amqp://rmq:rmq@localhost:5672/%2f");

        let jwt_secret = config("IDENTITY_SECRET_KEY")?;

        let config = Config {
            listen_port,
            bonus_service_address,
            flight_service_address,
            ticket_service_address,
            rmq_address,

            jwt_secret,
        };

        Ok(config)
    }
}
