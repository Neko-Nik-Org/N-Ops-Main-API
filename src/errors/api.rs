use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use super::service::ServiceError;


pub type ApiResponse = Result<HttpResponse, ServiceError>;


// ------- Implementations ------- //


impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::Redis(_) => StatusCode::BAD_GATEWAY,
            ServiceError::SerDe(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::Reqwest(_) => StatusCode::BAD_GATEWAY,
            ServiceError::Pgsql(_) => StatusCode::EXPECTATION_FAILED,
            ServiceError::LettreAddress(_) => StatusCode::BAD_REQUEST,
            ServiceError::LettreBase(_) => StatusCode::EXPECTATION_FAILED,
            ServiceError::LettreSmtp(_) => StatusCode::GONE,
            ServiceError::RandomGeneration(_) => StatusCode::EXPECTATION_FAILED,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ServiceError::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            ServiceError::Forbidden(_) => StatusCode::FORBIDDEN,
            ServiceError::Gone(_) => StatusCode::GONE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        // Only safe information goes to the client.
        let message = match self {
            ServiceError::BadRequest(message)
            | ServiceError::NotFound(message)
            | ServiceError::Unprocessable(message)
            | ServiceError::PreconditionFailed(message)
            | ServiceError::Unauthorized(message)
            | ServiceError::Forbidden(message)
            | ServiceError::Gone(message) => message.as_str(),

            // Hidden internal errors
            ServiceError::SerDe(_) => "E121: Error parsing data",
            ServiceError::Reqwest(_) => "E141: Upstream API service error",
            ServiceError::Redis(_) => "E102: Cache Database error",
            ServiceError::Pgsql(_) => "E112: SQL Database error",
            ServiceError::LettreAddress(_) => "E122: Invalid email address",
            ServiceError::LettreBase(_) => "E131: Email sending error",
            ServiceError::LettreSmtp(_) => "E132: SMTP server error",
            ServiceError::RandomGeneration(_) => "E123: Random number generation error",
        };

        // Full internal error goes to logs.
        tracing::error!(
            error = ?self,
            status = %status,
            message = %message,
            "Application service error"
        );

        // Extract the error code from the message.
        let error_code = message
            .split(':')
            .next()
            .unwrap_or("E000");

        HttpResponse::build(status).json(serde_json::json!({
            "error": message,
            "code": error_code,
            "status": status.as_u16()
        }))
    }
}
