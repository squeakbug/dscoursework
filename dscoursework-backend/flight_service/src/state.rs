use std::boxed::Box;

use crate::{
    app::api::auth::JwtValidator,
    app::service::flight_service::FlightService,
    config
};

pub struct AppState {
    pub person_service: Box<dyn FlightService + Sync + Send>,
    pub jwt_validator: JwtValidator,
    pub config: config::Config,
}
