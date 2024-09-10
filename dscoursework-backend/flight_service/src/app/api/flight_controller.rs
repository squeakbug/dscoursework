use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder, Result};
use actix_web_validator::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::api::jwk_auth::AuthenticationGuard;

use crate::{
    state::AppState,
    app::api::error::*,
};

#[derive(Serialize, Deserialize, Validate)]
pub struct GetRequestQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub flight_number: Option<String>,
}

use tracing::info;

#[get("/flights")]
pub async fn list(
    state: Data<AppState>, 
    query: web::Query<GetRequestQuery>
) -> Result<impl Responder, ErrorResponse> {
    info!("uery: {:?}", "Hello workd!");
    state
        .person_service
        .get_flights(query.page, query.size, query.flight_number.clone())
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(err);
            ErrorResponse::map_io_error(err)
        })
        .map(|persons| HttpResponse::Ok().json(persons))
}

#[derive(Serialize, Deserialize, Validate)]
pub struct GetRequestPath {
    pub id: i32,
}

#[get("/flights/{id}")]
pub async fn get_id(
    state: Data<AppState>, 
    _: AuthenticationGuard, 
    path: Path<GetRequestPath>
) -> Result<impl Responder, ErrorResponse> {
    let id = path.id;
    state
        .person_service
        .get_flight(id)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(err);
            ErrorResponse::map_io_error(err)
        })
        .map(|person| HttpResponse::Ok().json(person))
}
