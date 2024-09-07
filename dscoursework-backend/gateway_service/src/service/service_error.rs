use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Clone, Debug, Display, Error, Serialize)]
pub enum ServiceError {
    #[allow(unused)]
    #[display(fmt = "not found error")]
    NotFoundError,

    #[allow(unused)]
    #[display(fmt = "bad request")]
    BadClientData,

    #[allow(unused)]
    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "not implemented")]
    #[allow(unused)]
    NotImplemented,

    #[display(fmt = "flight service unavailable")]
    #[allow(unused)]
    FlightServiceUnavailable,

    #[display(fmt = "ticket service unavailable")]
    #[allow(unused)]
    TicketServiceUnavailable,

    #[display(fmt = "bonus service unavailable")]
    #[allow(unused)]
    BonusServiceUnavailable,

    #[allow(unused)]
    #[display(fmt = "internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, ServiceError>;
