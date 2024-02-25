use std::env;
use std::fmt::{Debug, Display, Formatter};

use deadpool_lapin::{Manager, Pool};
use lapin::ConnectionProperties;
use log::info;
use tokio_amqp::LapinTokioExt;

use crate::rmq::rmq_listen;

mod rmq;

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Server is starting. Hold on tight while we're getting ready.");

    let bind_mq_host = config_default("LISTEN_MQ_ADDRESS", "amqp://rmq:rmq@rabbitmq:5672/%2f");

    info!("bind_mq_host = {}", bind_mq_host);

    let manager = Manager::new(bind_mq_host, ConnectionProperties::default().with_tokio());
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("cannot create pool");

    let _ = rmq_listen(pool.clone()).await;

    Ok(())
}
