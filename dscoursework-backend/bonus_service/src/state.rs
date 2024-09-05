use std::{
    sync::Arc,
    boxed::Box,
};

use crate::{
    app::api::auth::JwtValidator,
    app::{
        service::privilege_service::PrivilegeService,
        repository::statistics_repository::StatisticsRepository,
    },
    config,
};

pub struct AppState {
    pub privilege_service: Box<dyn PrivilegeService + Sync + Send>,
    pub statistics_repository: Arc<StatisticsRepository>,
    pub jwt_validator: JwtValidator,
    pub config: config::Config,
}
