use lettre::address::AddressError as LettreAddressError;
use lettre::transport::smtp::Error as LettreSmtpError;
use lettre::error::Error as LettreBaseError;
use getrandom::Error as GetRandomError;
use super::pgsql::PgError;
use redis::RedisError;
use thiserror::Error;


pub type ServiceResult<T> = Result<T, ServiceError>;


#[derive(Error, Debug)]
pub enum ServiceError {
    // -----------------------------
    // Infrastructure / library
    // -----------------------------

    #[error("JSON error: {0}")]
    SerDe(#[from] serde_json::Error),

    #[error("HTTP Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("PostgreSQL error: {0}")]
    Pgsql(#[from] PgError),

    #[error("Lettre SMTP error: {0}")]
    LettreSmtp(#[from] LettreSmtpError),

    #[error("Lettre base error: {0}")]
    LettreBase(#[from] LettreBaseError),

    #[error("Lettre address error: {0}")]
    LettreAddress(#[from] LettreAddressError),

    #[error("Random number generation error: {0}")]
    RandomGeneration(#[from] GetRandomError),

    // -----------------------------
    // Application-specific errors
    // -----------------------------

    #[error("Precondition failed: {0}")]
    PreconditionFailed(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unprocessable entity: {0}")]
    Unprocessable(String),

    #[error("Authorization failed: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Gone: {0}")]
    Gone(String),
}
