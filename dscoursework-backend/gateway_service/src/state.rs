use std::boxed::Box;

use deadpool_lapin::Pool;

use shared::auth::JwtValidator;
use crate::{
    config::Config, 
    service::gateway_service::GatewayService
};

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub gateway_service: Box<dyn GatewayService + Sync + Send>,
    pub jwt_validator: JwtValidator,
    pub config: Config,
    pub mq_pool: Pool,
}
