use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use std::time::Duration;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use endpoint::retryer_middleware::init_rmq_listen;
use futures::lock::Mutex;
use failsafe::backoff::Exponential;
use failsafe::failure_policy::ConsecutiveFailures;
use failsafe::StateMachine;
use failsafe::{backoff, failure_policy, Config};
use log::info;
use reqwest::Client;

use lapin::ConnectionProperties;
use deadpool_lapin::{Manager, Pool};
use service::service_error;

use crate::endpoint::auth_controller::*;
use crate::endpoint::gateway_controller::*;
use crate::service::gateway_service_impl::GatewayServiceImpl;
use crate::state::AppState;

mod config;
mod endpoint;
mod models;
mod service;
mod state;

fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(flights_list)
        .service(tickets_list)
        .service(ticket_create)
        .service(ticket_get)
        .service(ticket_delete)
        .service(get_user_bonuses)
        .service(bonuses_status)
        .service(oauth_login)
        .service(oauth_callback);
}

fn circuit_breaker() -> StateMachine<ConsecutiveFailures<Exponential>, ()> {
    let backoff = backoff::exponential(Duration::from_secs(10), Duration::from_secs(60));

    let policy = failure_policy::consecutive_failures(3, backoff);

    Config::new().failure_policy(policy).build()
}

async fn rmq_listen(pool: Pool) -> service_error::Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        println!("connecting rmq consumer...");
        return init_rmq_listen(pool.clone()).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "RUST_LOG=info");
    }
    env_logger::init();

    let config = config::Config::init().expect("Bad read config");

    info!("Server is starting. Hold on tight while we're getting ready.");

    info!("Initialising HTTP server ...");

    let listen_address = config.listen_address.clone();
    info!("listen_address = {}", &listen_address);
    info!("bonus_address = {}", &config.bonus_service_address);
    info!("flight_address = {}", &config.flight_service_address);
    info!("ticket_address = {}", &config.ticket_service_address);

    let mq_host = "amqp://rmq:rmq@rabbitmq:5672/%2f";
    let tx_manager = Manager::new(mq_host, ConnectionProperties::default());
    let rx_manager = Manager::new(mq_host, ConnectionProperties::default());
    let tx_pool: Pool = deadpool::managed::Pool::builder(tx_manager)
        .max_size(10)
        .build()
        .expect("cannot create pool");
    let rx_pool: Pool = deadpool::managed::Pool::builder(rx_manager)
        .max_size(10)
        .build()
        .expect("cannot create pool");

    info!("mq_host = {}", mq_host);

    let token_storage = state::HashMapSyncContainer(Arc::new(Mutex::new(HashMap::new())));

    let http_server = HttpServer::new(move || {
        let state = AppState {
            gateway_service: Box::new(GatewayServiceImpl {
                flight_base_path: config.flight_service_address.clone(),
                ticket_base_path: config.ticket_service_address.clone(),
                bonus_base_path: config.bonus_service_address.clone(),
                client: Client::new(),
                circuit_breaker: circuit_breaker(),
            }),
            user_tokens: token_storage.clone(),
            config: config.clone(),
            mq_pool: tx_pool.clone(),
        };

        App::new()
            .app_data(web::Data::new(state))
            .wrap(Logger::default())
            .route("/manage/health", web::get().to(HttpResponse::Ok))
            .service(web::scope("/api/v1").configure(service_config))
    })
    .bind(&listen_address)
    .unwrap_or_else(|_| panic!("Could not bind on '{}'", &listen_address))
    .run();

    let _ = http_server.await;
    rmq_listen(rx_pool).await.map_err(|_| {
        Error::from(ErrorKind::UnexpectedEof)
    })
}
