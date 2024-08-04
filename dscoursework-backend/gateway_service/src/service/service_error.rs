use derive_more::Display;

#[derive(Debug, Display)]
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

    #[display(fmt = "service unavailable")]
    #[allow(unused)]
    ServiceUnavailable(String),

    #[allow(unused)]
    #[display(fmt = "internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, ServiceError>;
