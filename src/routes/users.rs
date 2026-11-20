use crate::features::users::{self, SessionUser, UpdateProfileReqDto, ListUsersReqDto, CreateUserReqDto};
use crate::cache::redis_cache::{get_redis_cache, set_redis_cache, delete_redis_cache};
use actix_web::{get, post, put, patch, delete, web, HttpResponse};
use crate::errors::{ApiResponse, ServiceError};
use crate::state::AppState;
use crate::security::otp;
use crate::messaging;



#[get("/profile")]
pub async fn get_profile(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the UI info for the session user
    let ui_info = users::fetch_user_profile(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(ui_info))
}


#[get("/ui-info")]
pub async fn get_ui_info(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the UI info for the session user
    let ui_info = users::fetch_user_ui_info(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(ui_info))
}


#[post("/ui-info")]
pub async fn set_ui_info(session_user: web::ReqData<SessionUser>, data: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let ui_info = data.into_inner(); 

    // Update the UI info for the session user
    let updated_ui_info = users::reset_user_ui_info(&state.pg_pool, session_user.user_id, &ui_info).await?;
    if updated_ui_info == 0 {
        return Err(ServiceError::BadRequest("Failed to update UI info".into()));
    }

    Ok(HttpResponse::Accepted().finish())
}


#[get("/details")]
pub async fn get_user_details(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the user details for the session user
    let user_details = users::fetch_user_details(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(user_details))
}


#[post("/details")]
pub async fn set_user_details(session_user: web::ReqData<SessionUser>, data: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_details = data.into_inner(); 

    // Update the user details for the session user
    let updated_user_details = users::reset_user_details(&state.pg_pool, session_user.user_id, &user_details).await?;
    if updated_user_details == 0 {
        return Err(ServiceError::BadRequest("Failed to update user details".into()));
    }

    Ok(HttpResponse::Accepted().finish())
}


#[get("/org-info")]
pub async fn get_org_info(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Fetch the organization info for the session user
    let org_info = users::fetch_org_info(&state.pg_pool, session_user.organization_id).await?;

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
    users::update_user_email_verified(&state.pg_pool, session_user.organization_id, session_user.user_id, true).await?;

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
    let exists = users::check_username_exists(&state.pg_pool, &username).await?;

    if exists {
        Ok(HttpResponse::Conflict().body("Username is already taken"))
    } else {
        Ok(HttpResponse::Ok().body("Username is available"))
    }
}


#[post("/change-password")]
pub async fn self_password(session_user: web::ReqData<SessionUser>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_password_base64 = body.into_inner();

    // Update the user's password in the database
    users::update_user_password(&state, session_user.organization_id, session_user.user_id, &new_password_base64).await?;

    Ok(HttpResponse::Ok().body("Password changed successfully"))
}


#[post("/change-username")]
pub async fn self_user_name_update(session_user: web::ReqData<SessionUser>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_username = body.into_inner();

    // Update the user's username in the database
    users::update_user_username(&state, session_user.organization_id, session_user.user_id, &new_username).await?;

    Ok(HttpResponse::Ok().body("Username changed successfully"))
}


#[delete("/deactivate")]
pub async fn self_deactivate(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Delete the user session from Redis
    let session_key = format!("session:{}", session_user.session_id);
    delete_redis_cache(&state.redis_cache, &session_key).await?;

    // Deactivate the user's account in the database
    let result = users::deactivate_user(&state.pg_pool, session_user.organization_id, session_user.user_id).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().body("User not found! Maybe already deactivated or deleted"));
    }

    Ok(HttpResponse::Ok().body("Your account has been deactivated successfully, contact admin to reactivate if needed"))
}


#[put("/profile")]
pub async fn update_profile(session_user: web::ReqData<SessionUser>, body: web::Json<UpdateProfileReqDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let update_request = body.into_inner();

    // Update the user's profile in the database
    users::update_user_profile(&state, session_user.user_id, &update_request).await?;

    Ok(HttpResponse::Ok().body("Profile updated successfully"))
}


#[get("/list")]
pub async fn list_users(session_user: web::ReqData<SessionUser>, query: web::Query<ListUsersReqDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let list_users_query = query.into_inner();

    session_user.has_all_permissions(&["user:view"])?;

    // Fetch the list of users from the database
    let users_list = users::list_all_users(&state.pg_pool, session_user.organization_id, &list_users_query).await?;

    Ok(HttpResponse::Ok().json(users_list))
}


#[get("/fetch/{user_id}")]
pub async fn get_user(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();

    session_user.has_all_permissions(&["user:view"])?;

    // Fetch the user by ID from the database
    let user = users::fetch_user_by_id_listing(&state.pg_pool, session_user.organization_id, user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}


#[post("/create")]
pub async fn create_user(session_user: web::ReqData<SessionUser>, body: web::Json<CreateUserReqDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let create_request = body.into_inner();

    session_user.has_all_permissions(&["user:view", "user:create", "user:role:view"])?;

    // Note: If the user has user:create permission, they can bypass tag and role level restrictions when creating a new user
    // So, make sure granting this permission is done carefully and only to trusted users!

    // Create the user in the database
    users::create_user(&state, &session_user, &create_request).await?;

    // Send welcome email to the newly created user
    messaging::email::send_welcome_email(&create_request.display_name, &create_request.primary_email).await?;

    Ok(HttpResponse::Ok().body("User created successfully and a welcome email sent"))
}


#[patch("/verify-email/{user_id}")]
pub async fn set_verify_email(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();

    session_user.has_all_permissions(&["user:view", "user:edit"])?;

    // Admin verify the user's email in the database
    let result = users::update_user_email_verified(&state.pg_pool, session_user.organization_id, user_id, true).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().body("User not found in the organization"));
    }

    Ok(HttpResponse::NoContent().finish())
}


#[put("/reset/password/{user_id}")]
pub async fn reset_password(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();
    let new_password = body.into_inner();   // Base64 encoded new password

    session_user.has_all_permissions(&["user:view", "user:edit"])?;

    // Admin reset - Update the user's password in the database
    users::update_user_password(&state, session_user.organization_id, user_id, &new_password).await?;

    Ok(HttpResponse::NoContent().finish())
}


#[delete("/delete/{user_id}")]
pub async fn delete_user(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();

    session_user.has_all_permissions(&["user:view", "user:edit"])?;

    // This will permanently delete the user and its related data from the organization, so use with caution.
    // Including Shared Notes, Private Notes, and any other user-specific data

    // Admin delete the user from the database
    let result = users::delete_user(&state.pg_pool, session_user.organization_id, user_id).await?;
    if result == 0 {
        return Ok(HttpResponse::ImUsed().body("User already deleted and not present in the organization"));
    }

    Ok(HttpResponse::NoContent().finish())
}


#[patch("/deactivate/{user_id}")]
pub async fn deactivate_user(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();

    session_user.has_all_permissions(&["user:view", "user:edit"])?;

    // Admin deactivate the user in the database
    let result = users::deactivate_user(&state.pg_pool, session_user.organization_id, user_id).await?;
    if result == 0 {
        return Ok(HttpResponse::ImUsed().body("User already deactivated and not present in the organization"));
    }

    Ok(HttpResponse::NoContent().finish())
}


#[put("/change-username/{user_id}")]
pub async fn change_username(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();
    let new_username = body.into_inner();

    session_user.has_all_permissions(&["user:view", "user:edit"])?;

    // Admin change the user's username in the database
    users::update_user_username(&state, session_user.organization_id, user_id, &new_username).await?;

    Ok(HttpResponse::NoContent().finish())
}
