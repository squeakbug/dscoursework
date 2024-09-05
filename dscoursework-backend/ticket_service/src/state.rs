use std::boxed::Box;

use crate::{
    app::api::auth::JwtValidator,
    app::service::ticket_service::TicketService,
    config,
};

pub struct AppState {
    pub ticket_service: Box<dyn TicketService + Sync + Send>,
    pub jwt_validator: JwtValidator,
    pub config: config::Config,
}
