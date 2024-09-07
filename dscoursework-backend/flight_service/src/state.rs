use std::{
    sync::Arc,
    boxed::Box,
};

use crate::{
    app::{
        api::auth::JwtValidator,
        service::flight_service::FlightService,
        repository::statistics_repository::StatisticsRepository,
    },
    config
};

pub struct AppState {
    pub person_service: Box<dyn FlightService + Sync + Send>,
    pub statistics_repository: Arc<StatisticsRepository>,
    pub jwt_validator: JwtValidator,
    pub config: config::Config,
}
