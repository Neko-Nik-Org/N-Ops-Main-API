use actix_web::{
    dev::{
        ServiceRequest,
        ServiceResponse
    },
    body::MessageBody,
    middleware::Next,
    HttpResponse,
    HttpMessage,
    Error,
    web,
};
use crate::{features::users::SessionUser, state::AppState};
use crate::cache::redis_cache::get_redis_cache;



/// Check for valid session based on Session-ID cookie and x-csrf-token header
/// Inserts SessionUser into request extensions if valid
/// Returns true if valid, false otherwise
async fn session_check(req: &ServiceRequest) -> bool {
    // Extract session token from header
    let session_token = req
        .headers()
        .get("x-nops-token")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string());

    // If session token is missing, fail
    if session_token.is_none() {
        return false;
    }

    // Check cache for session
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let cache_key = format!("session:{}", session_token.as_ref().unwrap());

    // Get cached session user from Redis
    let session_user: Option<SessionUser> = get_redis_cache(&state.redis_cache, &cache_key).await.unwrap();
    if session_user.is_none() {
        return false;
    }

    // Insert user into request extensions for further use
    req.extensions_mut().insert(session_user.unwrap());

    return true;
}


/// Authentication middleware
/// Checks for valid session
/// Short-circuits with 401 Unauthorized if checks fail
/// Otherwise calls the next service in the chain
pub async fn auth_check<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse, Error>
    where B: MessageBody + 'static
{
    // See if session is valid
    if !session_check(&req).await {
        tracing::warn!("Session check failed for request {} {}", req.method(), req.path());

        // Short-circuit and return 401 Unauthorized
        let resp = HttpResponse::Unauthorized()
            .append_header(("content-type", "text/plain; charset=utf-8"))
            .body("Unauthorized: Invalid session");

        // Convert into a ServiceResponse with a boxed body to satisfy types
        return Ok(req.into_response(resp).map_into_boxed_body());
    }

    // authorized -> call the next service
    let res = next.call(req).await?;
    Ok(res.map_into_boxed_body())
}
