use std::{
    sync::Arc,
    boxed::Box,
};

use crate::{
    app::{
        api::auth::JwtValidator,
        service::ticket_service::TicketService,
        repository::statistics_repository::StatisticsRepository,
    },
    config,
};

pub struct AppState {
    pub ticket_service: Box<dyn TicketService + Sync + Send>,
    pub statistics_repository: Arc<StatisticsRepository>,
    pub jwt_validator: JwtValidator,
    pub config: config::Config,
}
