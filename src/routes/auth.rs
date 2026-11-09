use crate::features::users::{SessionUser, LoginReqDto, create_user_session};
use crate::cache::redis_cache::{set_redis_cache, delete_redis_cache};
use actix_web::{delete, get, post, web, HttpResponse, HttpRequest};
use crate::security::cf_turnstile::verify_turnstile_response;
use crate::errors::{ApiResponse, ServiceError};
use crate::state::AppState;



#[post("/login")]
pub async fn user_login(request: HttpRequest, login_request: web::Json<LoginReqDto>, state: web::Data<AppState>) -> ApiResponse {
    // Extract the CloudFlare Turnstile token from the request headers
    let turnstile_token = request
        .headers()
        .get("cf-turnstile-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    // Extract the client's IP address from the request headers
    let client_ip = request
        .headers()
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    let login_request = login_request.into_inner();

    // Validate everything
    if turnstile_token.is_empty() || login_request.user_name.is_empty() || login_request.password.is_empty() || client_ip.is_empty() {
        return Err(ServiceError::BadRequest("Missing required fields".into()));
    }

    // Check if the CloudFlare Turnstile token is valid
    verify_turnstile_response(turnstile_token).await?;

    // Create a new user session
    let session = create_user_session(&state, &login_request, client_ip).await?;

    // 3 Hours per session

    Ok(HttpResponse::NotImplemented().body("Session creation not fully implemented"))
}


#[get("/session")]
pub async fn get_session_handler(session_user: web::ReqData<SessionUser>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    // TODO: Fetch fresh info to UI?

    Ok(HttpResponse::Ok().body(format!("Hello, {}", session_user.user_name)))
}
