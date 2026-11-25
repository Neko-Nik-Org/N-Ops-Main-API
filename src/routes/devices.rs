use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::features::devices;
use crate::state::AppState;



#[post("/generate")]
pub async fn generate_device_token(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Generate the device token using the session user's context
    let token = devices::generate_device_token(&state, session_user).await?;

    Ok(HttpResponse::Ok().json(token))
}


#[delete("/revoke")]
pub async fn delete_device_token(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Revoke the user's device using the provided data and the session user's context
    devices::revoke_device_token(&state, session_user).await?;

    Ok(HttpResponse::Ok().finish())
}


#[delete("/delete/{session_id}")]
pub async fn revoke_user_device(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let session_id = path.into_inner();

    // Revoke the user's device using the provided data and the session user's context
    devices::delete_device(&state.pg_pool, session_user.user_id, session_id).await?;

    Ok(HttpResponse::NoContent().finish())
}


#[get("/check/device-token")]
pub async fn check_device_token(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Check if the device token is valid for the session user
    let still_exists = devices::check_device_token(&state, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(still_exists))
}


#[get("/get/{session_id}")]
pub async fn get_device(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let session_id = path.into_inner();

    // Get the device using the provided data and the session user's context
    let device = devices::get_device(&state.pg_pool, session_user.user_id, session_id).await?;

    Ok(HttpResponse::Ok().json(device))
}


#[get("/list")]
pub async fn list_all_devices(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Note: How many devices one user can have?
    // Max 20 or even 50 !? So, no need pagination or complex filtering for now

    // Get the list of all devices for the session user's organization (No device-specific filtering applied)
    let devices = devices::list_all_devices(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(devices))
}
