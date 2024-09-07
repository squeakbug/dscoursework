use std::sync::Arc;

use actix::{
    sync::SyncArbiter,
    Addr,
};
use actix_web::{
    middleware::Logger,
    App, web, HttpServer, HttpResponse,
};
use app::repository::statistics_repository::StatisticsRepository;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;
use rdkafka::{producer::FutureProducer, ClientConfig};
use tracing::info;

use crate::{
    state::AppState,
    config::ConfigError,
    app::{
        service::ticket_service_impl::TicketServiceImpl,
        api::{
            ticket_controller,
            auth::JwtValidator,
        },
        repository::{
            database_executor::DatabaseExecutor,
            ticket_repository::*,
        },
    },
};

pub mod app;
pub mod config;
pub mod schema;
pub mod state;

fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(ticket_controller::list)
        .service(ticket_controller::create)
        .service(ticket_controller::get_id)
        .service(ticket_controller::delete)
        .service(ticket_controller::patch_id);
}

fn start_db_executor(cfg: &config::Config) -> Result<Addr<DatabaseExecutor>, ConfigError> {
    info!("Initialising database connection pool ...");
    let db_url = cfg.database_url.clone();

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(manager).map_err(|_| ConfigError {
        message: String::from("Failed to initialise DB pool"),
    })?;

    Ok(SyncArbiter::start(2, move || DatabaseExecutor(pool.clone())))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::init().expect("Bad read config");

    info!("Server is starting. Hold on tight while we're getting ready.");

    let listen_port = config.listen_port.clone();
    info!("listen_port = {}", &listen_port);

    let db_addr = start_db_executor(&config).unwrap();
    let ticket_repository = TicketRepositoryImpl { db_addr };
    let ticket_service = TicketServiceImpl {
        ticket_repository: Box::new(ticket_repository),
    };

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_bootstrap_servers)
        .create()
        .expect("Producer creation failed");
    let statistics_repository = Arc::new(StatisticsRepository::new(producer));

    let jwt_secret = config.jwt_secret.clone();
    let listen_port = config.listen_port.parse::<u16>().expect("Invalid listen port");
    HttpServer::new(move || {
        let jwt_validator = JwtValidator::new(&jwt_secret);
        let state = AppState {
            ticket_service: Box::new(ticket_service.clone()),
            config: config.clone(),
            statistics_repository: statistics_repository.clone(),
            jwt_validator,
        };

        App::new()
            .route("/manage/health", web::get().to(HttpResponse::Ok))
            .service(web::scope("/api/v1").configure(service_config))
            .wrap(Logger::default())
            .app_data(web::Data::new(state))
    })
    .bind(("0.0.0.0", listen_port))
    .unwrap_or_else(|_| panic!("Could not bind on '{}'", listen_port))
    .run()
    .await
}
