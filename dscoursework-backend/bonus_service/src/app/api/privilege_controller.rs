use actix_web::web::Data;
use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use actix_web_validator::Path;
use serde::Deserialize;
use validator::Validate;

use crate::{
    state::AppState,
    app::{
        api::{
            error::*,
            jwk_auth::AuthenticationGuard,
        },
        models,
    },
};

#[get("/privileges")]
pub async fn list_privileges(
    state: Data<AppState>,
    auth_guard: AuthenticationGuard,
) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .list_privileges(Some(auth_guard.claims.nickname))
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(err);
            ErrorResponse::map_io_error(err)
        })
        .map(|privileges| HttpResponse::Ok().json(privileges))
}

#[post("/bonuses")]
pub async fn create_bonus(
    state: Data<AppState>,
    _: AuthenticationGuard,
    bonus: web::Json<models::PrivilegeRequest>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .create_bonus(&bonus)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(err);
            ErrorResponse::map_io_error(err)
        })
        .map(|privileges| HttpResponse::Ok().json(privileges))
}

#[derive(Deserialize, Validate)]
pub struct DeleteBonusPath {
    #[serde(rename = "ticketUid")]
    pub ticket_uid: uuid::Uuid,
}

#[delete("/bonuses/{ticketUid}")]
pub async fn delete_bonus(
    state: Data<AppState>,
    auth_guard: AuthenticationGuard,
    path: Path<DeleteBonusPath>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .delete_bonus(auth_guard.claims.nickname, path.ticket_uid)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(err);
            ErrorResponse::map_io_error(err)
        })
        .map(|privilege| HttpResponse::Ok().json(privilege))
}

#[derive(Deserialize, Validate)]
pub struct PrivilegeHistoryQuery {
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(rename = "ticketUid")]
    pub ticket_uid: Option<uuid::Uuid>,
}

#[get("/privilege_history")]
pub async fn list_privilege_history(
    state: Data<AppState>,
    auth_guard: AuthenticationGuard,
    query: web::Query<PrivilegeHistoryQuery>,
) -> Result<impl Responder, ErrorResponse> {
    state
        .privilege_service
        .get_privilege_history(Some(auth_guard.claims.nickname), query.ticket_uid)
        .await
        .map_err(|err| {
            let repo = &state.statistics_repository;
            let _ = repo.create_error_message(err);
            ErrorResponse::map_io_error(err)
        })
        .map(|privilege| HttpResponse::Ok().json(privilege))
}
