use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{delete, get, patch, post, web, HttpResponse, HttpResponseBuilder, Responder, Result};
use actix_web_validator::Path;
use serde::Deserialize;
use serde_derive::Serialize;
use validator::Validate;

use shared::auth::JwtAuthGuard;
use crate::app::{
    api::{
        error::*,
        state::AppState,
    },
    dto_models,
};

#[derive(Serialize, Deserialize)]
pub struct ListTicketsQuery {
    pub flight_number: Option<String>,
}

#[get("/tickets")]
pub async fn list(
    state: Data<AppState>, 
    auth_guard: JwtAuthGuard, 
    query: web::Query<ListTicketsQuery>
) -> Result<impl Responder, ErrorResponse> {
    state
        .ticket_service
        .get_tickets(Some(auth_guard.claims.sub), query.flight_number.clone())
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|tickets| HttpResponse::Ok().json(tickets))
}

#[post("/tickets")]
pub async fn create(
    state: Data<AppState>,
    _auth_guard: JwtAuthGuard,
    ticket: web::Json<dto_models::TicketCreateRequest>
) -> Result<impl Responder, ErrorResponse> {
    state
        .ticket_service
        .create_ticket(&ticket)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|new_id| HttpResponseBuilder::new(StatusCode::CREATED).json(new_id))
}

#[get("/tickets/{ticketUid}")]
pub async fn get_id(
    state: Data<AppState>, 
    _auth_guard: JwtAuthGuard, 
    path: Path<DeleteRequest>
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .get_ticket(ticket_uid)
        .await
        .map_err(ErrorResponse::map_io_error)
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
    _auth_guard: JwtAuthGuard, 
    path: Path<DeleteRequest>
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .delete_ticket(ticket_uid)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|_| HttpResponseBuilder::new(StatusCode::NO_CONTENT).finish())
}

#[patch("/tickets/{ticketUid}")]
pub async fn patch_id(
    state: Data<AppState>,
    _auth_guard: JwtAuthGuard,
    path: Path<DeleteRequest>,
    ticket: web::Json<dto_models::TicketRequest>,
) -> Result<impl Responder, ErrorResponse> {
    let ticket_uid = path.ticket_uid;
    state
        .ticket_service
        .edit_ticket(ticket_uid, &ticket)
        .await
        .map_err(ErrorResponse::map_io_error)
        .map(|ticket| HttpResponse::Ok().json(ticket))
}
