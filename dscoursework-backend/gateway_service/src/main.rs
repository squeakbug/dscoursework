use std::{
    time::Duration,
    io::{ErrorKind, Error},
};

use actix_web::{
    web, App,
    HttpResponse, HttpServer,
    middleware::Logger,
};
use failsafe::{
    backoff::{self, Exponential},
    failure_policy::ConsecutiveFailures,
    StateMachine, failure_policy, Config
};
use tracing::info;
use tracing_subscriber;
use reqwest::Client;

use lapin::ConnectionProperties;
use deadpool_lapin::{Manager, Pool};
use service::service_error;

use shared::auth::JwtValidator;
use crate::{
    api::{
        gateway_controller::*,
        retryer_middleware::init_rmq_listen,
    },
    service::gateway_service_impl::GatewayServiceImpl,
    state::AppState,
};

mod api;
mod config;
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
        .service(bonuses_status);
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
    tracing_subscriber::fmt::init();

    let config = config::Config::init().expect("Bad read config");

    info!("Server is starting. Hold on tight while we're getting ready.");

    info!("Initialising HTTP server ...");

    info!("listen_port = {}", &config.listen_port);
    info!("bonus_address = {}", &config.bonus_service_address);
    info!("flight_address = {}", &config.flight_service_address);
    info!("ticket_address = {}", &config.ticket_service_address);
    info!("rmq_address = {}", &config.rmq_address);

    let tx_manager = Manager::new(&config.rmq_address, ConnectionProperties::default());
    let rx_manager = Manager::new(&config.rmq_address, ConnectionProperties::default());
    let tx_pool: Pool = deadpool::managed::Pool::builder(tx_manager)
        .max_size(10)
        .build()
        .expect("cannot create pool");
    let rx_pool: Pool = deadpool::managed::Pool::builder(rx_manager)
        .max_size(10)
        .build()
        .expect("cannot create pool");

    let jwt_secret = config.jwt_secret.clone();
    let listen_port = config.listen_port.parse::<u16>().expect("Invalid listen port");
    let http_server = HttpServer::new(move || {
        let jwt_validator = JwtValidator::new(&jwt_secret);
        let state = AppState {
            gateway_service: Box::new(GatewayServiceImpl {
                flight_base_path: config.flight_service_address.clone(),
                ticket_base_path: config.ticket_service_address.clone(),
                bonus_base_path: config.bonus_service_address.clone(),
                client: Client::new(),
                circuit_breaker: circuit_breaker(),
            }),
            config: config.clone(),
            mq_pool: tx_pool.clone(),
            jwt_validator,
        };

        App::new()
            .app_data(web::Data::new(state))
            //.wrap_fn(validate_jwt)
            .route("/manage/health", web::get().to(HttpResponse::Ok))
            .service(web::scope("/api/v1").configure(service_config))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", listen_port))
    .unwrap_or_else(|_| panic!("Could not bind on '{}'", listen_port))
    .run();

    let _ = http_server.await;
    rmq_listen(rx_pool).await.map_err(|_| {
        Error::from(ErrorKind::UnexpectedEof)
    })
}
