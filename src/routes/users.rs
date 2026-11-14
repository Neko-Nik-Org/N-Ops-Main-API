use crate::features::users::{SessionUser, fetch_user_ui_info, fetch_user_details, reset_user_details, reset_user_ui_info, fetch_org_info, update_user_email_verified, check_username_exists};
use crate::cache::redis_cache::{get_redis_cache, set_redis_cache, delete_redis_cache};
use crate::errors::{ApiResponse, ServiceError};
use actix_web::{get, post, web, HttpResponse};
use crate::state::AppState;
use crate::security::otp;
use crate::messaging;



#[get("/ui-info")]
pub async fn get_ui_info(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the UI info for the session user
    let ui_info = fetch_user_ui_info(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(ui_info))
}


#[post("/ui-info")]
pub async fn set_ui_info(session_user: web::ReqData<SessionUser>, data: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let ui_info = data.into_inner(); 

    // Update the UI info for the session user
    let updated_ui_info = reset_user_ui_info(&state.pg_pool, session_user.user_id, &ui_info).await?;
    if updated_ui_info == 0 {
        return Err(ServiceError::BadRequest("Failed to update UI info".into()));
    }

    Ok(HttpResponse::Accepted().finish())
}


#[get("/details")]
pub async fn get_user_details(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the user details for the session user
    let user_details = fetch_user_details(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(user_details))
}


#[post("/details")]
pub async fn set_user_details(session_user: web::ReqData<SessionUser>, data: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_details = data.into_inner(); 

    // Update the user details for the session user
    let updated_user_details = reset_user_details(&state.pg_pool, session_user.user_id, &user_details).await?;
    if updated_user_details == 0 {
        return Err(ServiceError::BadRequest("Failed to update user details".into()));
    }

    Ok(HttpResponse::Accepted().finish())
}


#[get("/org-info")]
pub async fn get_org_info(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the organization info for the session user
    let org_info = fetch_org_info(&state.pg_pool, session_user.organization_id).await?;

    Ok(HttpResponse::Ok().json(org_info))
}


#[post("/verify-email/send")]
pub async fn send_verification_email(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    if session_user.is_email_verified {
        return Ok(HttpResponse::Conflict().body("Email is already verified"));
    }

    // Generate a verification code for the session user
    let verification_code = otp::generate_otp(7)?;

    // Store the verification code in the cache with an expiration time
    let otp_hash = otp::hash_otp(&verification_code);

    // Store the OTP hash in the cache with an expiration time

    // We use the session ID to create a unique cache key for the OTP
    // So that even if the user is logged in from multiple devices, each OTP is unique
    let otp_cache_key = format!("otp:self:verify:email:{}", session_user.session_id);

    // Store the user session in Redis with a 5-minute TTL
    set_redis_cache(&state.redis_cache, &otp_cache_key, &otp_hash, 5 * 60).await?;

    // Send the verification email to the user's email address
    messaging::email::send_verification_email(&session_user, &verification_code).await?;

    Ok(HttpResponse::Created().finish())
}


#[post("/verify-email/validate")]
pub async fn verify_email(session_user: web::ReqData<SessionUser>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let otp_code = body.into_inner();

    if session_user.is_email_verified {
        return Ok(HttpResponse::Conflict().body("Email is already verified"));
    }

    // Fetch the OTP hash from the cache and compare it with the provided OTP code
    let otp_cache_key = format!("otp:self:verify:email:{}", session_user.session_id);

    // Retrieve the cached OTP hash from the cache to validate the provided OTP code
    let cached_otp_hash: Option<String> = get_redis_cache(&state.redis_cache, &otp_cache_key).await?;
    if cached_otp_hash.is_none() {
        return Ok(HttpResponse::Gone().body("Verification code has expired, please request a new one"));
    }

    // Get Input and Cached OTP Hash for comparison
    let cached_otp_hash = cached_otp_hash.unwrap();
    let input_otp_hash = otp::hash_otp(&otp_code);

    // Compare the cached OTP hash with the input OTP hash to validate the OTP code
    if cached_otp_hash != input_otp_hash {
        // Delete the OTP from the cache as it is incorrect to prevent brute force attempts
        delete_redis_cache(&state.redis_cache, &otp_cache_key).await?;

        return Ok(HttpResponse::BadRequest().body("Verification code is incorrect, request a new one"));
    }

    // At this point, the OTP code is verified successfully
    // Mark the user's email as verified in the database
    update_user_email_verified(&state.pg_pool, session_user.user_id, true).await?;

    // Delete the OTP from the cache as it has been successfully used
    delete_redis_cache(&state.redis_cache, &otp_cache_key).await?;

    Ok(HttpResponse::Ok().body("Email verified successfully"))
}


#[get("/user-name/check/{username}")]
pub async fn check_username(path: web::Path<String>, state: web::Data<AppState>) -> ApiResponse {
    let username = path.into_inner();

    // Can be used to Change Self's username or for Admins to check if a username
    // is available before creating or updating a user/profile

    // Check if the username exists in the database
    let exists = check_username_exists(&state.pg_pool, &username).await?;

    if exists {
        Ok(HttpResponse::Conflict().body("Username is already taken"))
    } else {
        Ok(HttpResponse::Ok().body("Username is available"))
    }
}
