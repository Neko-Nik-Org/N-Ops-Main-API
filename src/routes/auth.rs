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
    let auth_info = create_user_session(&state, &login_request, client_ip).await?;

    // If 2FA is enabled for the user, do not create the session, require 2FA validation first
    if auth_info.has_email_2fa || auth_info.has_totp_2fa {
        // Generate a new temporary auth token for 2FA validation
        let new_token = uuid::Uuid::new_v4().to_string();
        let temporary_auth_token = format!("session:temp:{}", new_token);

        // Store the temporary auth token in Redis with a 15-minute TTL
        set_redis_cache(&state.redis_cache, &temporary_auth_token, &auth_info, 15 * 60).await?;

        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "2FA validation required",
            "is_2fa_required": true,
            "temporary_auth_token": new_token,
            "data": {
                "display_name": auth_info.display_name,
                "organization_name": auth_info.organization_name,
                "primary_email": auth_info.primary_email,
                "is_email_verified": auth_info.is_email_verified,
                "has_email_2fa": auth_info.has_email_2fa,
                "has_totp_2fa": auth_info.has_totp_2fa,
                "last_login_at": auth_info.last_login_at
            }
        })));
    }

    // Generate a Session struct
    let user_session = SessionUser::new(auth_info, login_request.remember_me);

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
