use crate::features::users::{SessionUser, LoginReqDto, DeviceRefreshReqDto, create_user_session, refresh_user_session, update_last_login_at, create_device_session};
use crate::features::devices::{DeviceLoginReqDto, create_device_refresh_token};
use crate::cache::redis_cache::{set_redis_cache, delete_redis_cache};
use actix_web::{delete, get, post, web, HttpResponse, HttpRequest};
use crate::security::cf_turnstile::verify_turnstile_response;
use crate::errors::{ApiResponse, ServiceError};
use crate::state::AppState;



#[post("/login")]
pub async fn user_login(request: HttpRequest, login_request: web::Json<LoginReqDto>, state: web::Data<AppState>) -> ApiResponse {
    let login_request = login_request.into_inner();

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

    // Validate everything
    if turnstile_token.is_empty() || login_request.user_name.is_empty() || login_request.password.is_empty() || client_ip.is_empty() {
        return Err(ServiceError::BadRequest("Missing required fields".into()));
    }

    // Check if the CloudFlare Turnstile token is valid
    verify_turnstile_response(turnstile_token).await?;

    // Create a new user session
    let auth_info = create_user_session(&state, &login_request, client_ip).await?;

    // Check if 2FA enforcement policies are enabled for the user
    let enforce_email_2fa = auth_info.enforce_email_2fa;
    let enforce_totp_2fa = auth_info.enforce_totp_2fa;
    let is_2fa_enforced = enforce_email_2fa || enforce_totp_2fa;

    // If 2FA is enabled for the user, do not create the session, require 2FA validation first
    if auth_info.has_email_2fa || auth_info.has_totp_2fa {
        // Generate a new temporary auth token for 2FA validation
        let new_token = uuid::Uuid::new_v4().to_string();
        let temp_session_key = format!("session:temp:{}", new_token);

        // Store the temporary auth token in Redis with a 15-minute TTL
        set_redis_cache(&state.redis_cache, &temp_session_key, &auth_info, 15 * 60).await?;

        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "2FA validation required",
            "is_2fa_required": true,
            "is_2fa_enforced": is_2fa_enforced,
            "temporary_auth_token": new_token,
            "data": {
                "display_name": auth_info.display_name,
                "organization_name": auth_info.organization_name,
                "primary_email": auth_info.primary_email,
                "is_email_verified": auth_info.is_email_verified,
                "has_email_2fa": auth_info.has_email_2fa,
                "has_totp_2fa": auth_info.has_totp_2fa,
                "enforce_email_2fa": enforce_email_2fa,
                "enforce_totp_2fa": enforce_totp_2fa,
                "last_login_at": auth_info.last_login_at
            }
        })));
    }

    // Update the last login timestamp for the user
    update_last_login_at(&state.pg_pool, auth_info.user_id).await?;

    // Generate a Session struct
    let user_session = SessionUser::new(auth_info, login_request.remember_me);
    let session_key = format!("session:{}", user_session.session_id);

    // Store the user session in Redis with a 3-hour TTL
    set_redis_cache(&state.redis_cache, &session_key, &user_session, 3 * 60 * 60).await?;

    // Return the session creation response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Session created successfully",
        "is_2fa_required": false,
        "is_2fa_enforced": is_2fa_enforced,
        "session_id": user_session.session_id,
        "data": {
            "user_id": user_session.user_id,
            "organization_id": user_session.organization_id,
            "permissions": user_session.permissions,
            "tags": user_session.tags,
            "user_name": user_session.user_name,
            "display_name": user_session.display_name,
            "organization_name": user_session.organization_name,
            "primary_email": user_session.primary_email,
            "is_email_verified": user_session.is_email_verified,
            "enforce_email_2fa": enforce_email_2fa,
            "enforce_totp_2fa": enforce_totp_2fa
        }
    })))
}


#[post("/device/login")]
pub async fn device_login(body: web::Json<DeviceLoginReqDto>, state: web::Data<AppState>) -> ApiResponse {
    let device_login_request = body.into_inner();

    // Create a new device refresh token for the login request
    let refresh_token = create_device_refresh_token(&state, device_login_request).await?;

    // Return the session creation response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Refresh token created successfully",
        "refresh_token": refresh_token
    })))
}


#[post("/device/refresh")]
pub async fn device_refresh(body: web::Json<DeviceRefreshReqDto>, state: web::Data<AppState>) -> ApiResponse {
    let device_refresh_request = body.into_inner();

    // Create a new device session using the provided refresh token (It will be same as the normal login session)
    let user_session = create_device_session(&state, &device_refresh_request).await?;

    // Return the session creation response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Session created successfully",
        "session_id": user_session.session_id,
        "data": {
            "user_id": user_session.user_id,
            "organization_id": user_session.organization_id,
            "permissions": user_session.permissions,
            "tags": user_session.tags,
            "user_name": user_session.user_name,
            "display_name": user_session.display_name,
            "organization_name": user_session.organization_name,
            "primary_email": user_session.primary_email,
            "is_email_verified": user_session.is_email_verified
        }
    })))
}


#[get("/session")]
pub async fn get_session(session_user: web::ReqData<SessionUser>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Return the current session information for the user,
    // including permissions and session capabilities

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "allow_refresh": session_user.allow_refresh,
        "allow_device_logins": session_user.allow_device_logins,
        "allowed_linux_users": session_user.allowed_linux_users,
        "permissions": session_user.permissions,
        "tags": session_user.tags,
        "can_approve_approvals": session_user.can_approve_approvals,
        "can_bypass_approvals": session_user.can_bypass_approvals
    })))
}


#[get("/verify")]
pub async fn verify_session(session_user: web::ReqData<SessionUser>) -> ApiResponse {
    let _ = session_user.into_inner();

    // Simple verification endpoint to check if the session is valid
    Ok(HttpResponse::NoContent().finish())
}


#[delete("/logout")]
pub async fn user_logout(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Delete the user session from Redis
    let session_key = format!("session:{}", session_user.session_id);
    delete_redis_cache(&state.redis_cache, &session_key).await?;

    Ok(HttpResponse::NoContent().finish())
}


#[post("/refresh")]
pub async fn refresh_session(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    let new_session_user = refresh_user_session(&state, &session_user).await?;

    update_last_login_at(&state.pg_pool, new_session_user.user_id).await?;

    // Return the session creation response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Session refreshed successfully",
        "session_id": new_session_user.session_id
    })))
}
