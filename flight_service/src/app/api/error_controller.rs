use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;

use crate::app::service::service_error::ServiceError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub message: String,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Display)]
#[display(fmt = "{:?}", errors)]
pub struct ErrorResponse {
    status_code: StatusCode,
    errors: Option<Vec<String>>,
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorPresenter {
            message: status_code.to_string(),
            errors: self.errors.clone(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl ErrorResponse {
    pub fn map_io_error(e: ServiceError) -> ErrorResponse {
        match e {
            ServiceError::BadClientData => ErrorResponse {
                status_code: StatusCode::BAD_REQUEST,
                errors: Some(vec![e.to_string()]),
            },
            ServiceError::Timeout => ErrorResponse {
                status_code: StatusCode::GATEWAY_TIMEOUT,
                errors: Some(vec![e.to_string()]),
            },
            ServiceError::NotImplemented => ErrorResponse {
                status_code: StatusCode::NOT_IMPLEMENTED,
                errors: Some(vec![e.to_string()]),
            },
            ServiceError::NotFoundError => ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                errors: Some(vec![e.to_string()]),
            },
            ServiceError::InternalError => ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                errors: Some(vec![e.to_string()]),
            },
        }
    }
}
