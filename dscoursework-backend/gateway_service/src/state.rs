use std::{
    sync::Arc,
    boxed::Box,
};

use deadpool_lapin::Pool;

use crate::{
    api::auth::JwtValidator,
    config::Config, 
    service::gateway_service::GatewayService,
    repository::statistics_repository::StatisticsRepository,
};

pub struct AppState {
    pub gateway_service: Box<dyn GatewayService + Sync + Send>,
    pub statistics_repository: Arc<StatisticsRepository>,
    pub jwt_validator: JwtValidator,
    pub config: Config,
    
    pub mq_pool: Pool,
}
