use crate::features::users::{SessionUser, fetch_user_ui_info, fetch_user_details, reset_user_details, reset_user_ui_info, fetch_org_info};
use crate::cache::redis_cache::{get_redis_cache, set_redis_cache, delete_redis_cache};
use crate::errors::{ApiResponse, ServiceError};
use actix_web::{get, post, web, HttpResponse};
use crate::state::AppState;
use crate::security::otp;



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

    // Generate a verification code for the session user
    let verification_code = otp::generate_otp(7)?;

    // Store the verification code in the cache with an expiration time
    

    Ok(HttpResponse::Accepted().finish())
}
