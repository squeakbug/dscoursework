use actix_web::web::Data;
use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use log::info;
use serde::Deserialize;
use validator::Validate;

use shared::auth::JwtAuthGuard;
use crate::{
    api::error::ErrorResponse,
    models,
    state::AppState,
};

#[derive(Deserialize, Validate)]
pub struct Info {
    #[validate(range(min = 1000))]
    page: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    size: Option<i32>,
}

#[get("/flights")]
pub async fn flights_list(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
    info: web::Query<Info>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .get_flights(auth_guard.token, info.page, info.size)
        .await
        .map(|flights| HttpResponse::Ok().json(flights))
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}

#[get("/tickets")]
pub async fn tickets_list(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .get_user_tickets(auth_guard.token, auth_guard.claims.sub)
        .await
        .map(|tickets| HttpResponse::Ok().json(tickets))
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}

#[post("/tickets")]
pub async fn ticket_create(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
    body: web::Json<models::TicketPurchaseRequest>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .buy_ticket(auth_guard.token, auth_guard.claims.sub, body.0)
        .await
        .map(|ticket| HttpResponse::Ok().json(ticket))
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}

#[derive(Deserialize, Validate)]
pub struct GetTicketPath {
    #[serde(rename = "ticketUid")]
    pub ticket_uid: uuid::Uuid,
}

#[get("/tickets/{ticketUid}")]
pub async fn ticket_get(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
    path: web::Path<GetTicketPath>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .get_ticket_by_uid(auth_guard.token, auth_guard.claims.sub, path.ticket_uid)
        .await
        .map(|ticket| HttpResponse::Ok().json(ticket))
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}

#[derive(Deserialize, Validate)]
pub struct DeleteTicketPath {
    #[serde(rename = "ticketUid")]
    pub ticket_uid: uuid::Uuid,
}

#[delete("/tickets/{ticketUid}")]
pub async fn ticket_delete(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
    path: web::Path<DeleteTicketPath>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .return_ticket(auth_guard.token, auth_guard.claims.sub, path.ticket_uid)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}

#[get("/me")]
pub async fn get_user_bonuses(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .get_user_info(auth_guard.token, auth_guard.claims.sub)
        .await
        .map(|info| HttpResponse::Ok().json(info))
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}

#[get("/privilege")]
pub async fn bonuses_status(
    state: Data<AppState>,
    auth_guard: JwtAuthGuard,
) -> Result<impl Responder, ErrorResponse> {
    state
        .gateway_service
        .get_privilege_with_history(auth_guard.token, auth_guard.claims.sub)
        .await
        .map(|info| HttpResponse::Ok().json(info))
        .map_err(|err| {
            let response = ErrorResponse::map_io_error(err);
            info!("{}", response.to_string());
            response
        })
}
