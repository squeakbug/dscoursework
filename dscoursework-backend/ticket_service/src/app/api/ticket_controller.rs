use actix_web::{
    http::StatusCode,
    web::Data,
    delete, get, patch, post, web, HttpResponse, HttpResponseBuilder, Responder, Result,
};
use actix_web_validator::Path;
use serde::Deserialize;
use serde_derive::Serialize;
use validator::Validate;

use crate::{
    state::AppState,
    app::{
        api::{
            error::*,
            jwk_auth::AuthenticationGuard,
        },
        dto_models,
    },
};

#[derive(Serialize, Deserialize)]
pub struct ListTicketsQuery {
    pub flight_number: Option<String>,
}

#[get("/tickets")]
pub async fn list(
    state: Data<AppState>, 
    auth_guard: AuthenticationGuard, 
    query: web::Query<ListTicketsQuery>
) -> Result<impl Responder, ErrorResponse> {
    state
        .ticket_service
        .get_tickets(Some(auth_guard.claims.nickname), query.flight_number.clone())
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(&err);
            ErrorResponse::map_io_error(err)
        })
        .map(|tickets| HttpResponse::Ok().json(tickets))
}

use tracing::info;

#[post("/tickets")]
pub async fn create(
    state: Data<AppState>,
    _auth_guard: AuthenticationGuard,
    ticket: web::Json<dto_models::TicketCreateRequest>
) -> Result<impl Responder, ErrorResponse> {
    info!("{:?}", ticket);
    state
        .ticket_service
        .create_ticket(&ticket)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(&err);
            ErrorResponse::map_io_error(err)
        })
        .map(|new_id| HttpResponseBuilder::new(StatusCode::CREATED).json(new_id))
}

#[get("/tickets/{ticketUid}")]
pub async fn get_id(
    state: Data<AppState>, 
    _auth_guard: AuthenticationGuard, 
    path: Path<DeleteRequest>
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .get_ticket(ticket_uid)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(&err);
            ErrorResponse::map_io_error(err)
        })
        .map(|ticket| HttpResponse::Ok().json(ticket))
}

#[derive(Deserialize, Validate)]
pub struct DeleteRequest {
    #[serde(rename = "ticketUid")]
    pub ticket_uid: uuid::Uuid,
}

#[delete("/tickets/{ticketUid}")]
pub async fn delete(
    state: Data<AppState>, 
    _auth_guard: AuthenticationGuard, 
    path: Path<DeleteRequest>
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .delete_ticket(ticket_uid)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(&err);
            ErrorResponse::map_io_error(err)
        })
        .map(|_| HttpResponseBuilder::new(StatusCode::NO_CONTENT).finish())
}

#[patch("/tickets/{ticketUid}")]
pub async fn patch_id(
    state: Data<AppState>,
    _auth_guard: AuthenticationGuard,
    path: Path<DeleteRequest>,
    ticket: web::Json<dto_models::TicketRequest>,
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .edit_ticket(ticket_uid, &ticket)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(&err);
            ErrorResponse::map_io_error(err)
        })
        .map(|ticket| HttpResponse::Ok().json(ticket))
}
