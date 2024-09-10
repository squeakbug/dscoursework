use std::fmt::{Debug, Display, Formatter};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_port: String,
    pub database_url: String,
    pub kafka_bootstrap_servers: String,

    pub authentik_jwks: String,
    pub authentik_user_info: String,
    pub authentik_openid_config: String,

    pub jwt_secret: String,
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
        let kafka_bootstrap_servers = config("KAFKA__BOOTSTRAP_SERVERS")?;

        let authentik_jwks = config("AUTHENTIK__JWKS")?;
        let authentik_user_info = config("AUTHENTIK__USER_INFO")?;
        let authentik_openid_config = config("AUTHENTIK__OPENID_CONFIG")?;

        let jwt_secret = config("IDENTITY_SECRET_KEY")?;

        let config = Config {
            listen_port,
            database_url,
            kafka_bootstrap_servers,

            authentik_jwks,
            authentik_user_info,
            authentik_openid_config,

            jwt_secret,
        };

        Ok(config)
    }
}
