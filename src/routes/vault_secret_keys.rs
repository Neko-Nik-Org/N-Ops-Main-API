use crate::features::vault_secret_keys::{self, SkNewDto, SkVersionDto, SkListingDto, SkDeleteDto};
use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create")]
pub async fn create_secret_key(session_user: web::ReqData<SessionUser>, data: web::Json<SkNewDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_sk_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:keys:view", "vault:keys:create"])?;
    session_user.has_tag_permission(&state, new_sk_data.tag_id).await?;

    // Create the new secret key using the provided data and the session user's context
    vault_secret_keys::create_new_sk(&state, &session_user, &new_sk_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[put("/update")]
pub async fn update_secret_key(session_user: web::ReqData<SessionUser>, data: web::Json<SkVersionDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_sk_version_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:keys:view", "vault:keys:edit"])?;
    session_user.has_tag_permission(&state, new_sk_version_data.tag_id).await?;

    // Create the new secret key using the provided data and the session user's context
    vault_secret_keys::make_new_sk_version(&state, &session_user, &new_sk_version_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[get("/list")]
pub async fn list_secret_keys(session_user: web::ReqData<SessionUser>, query: web::Query<SkListingDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let sk_query_params = query.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:keys:view", "vault:keys:edit"])?;
    session_user.has_tag_permission(&state, sk_query_params.tag_id).await?;

    
    // Create the new secret key using the provided data and the session user's context
    let secret_keys = vault_secret_keys::list_secret_keys(
        &state.pg_pool,
        session_user.organization_id,
        sk_query_params.project_id,
        sk_query_params.tag_id,
        sk_query_params.limit,
        sk_query_params.offset
    ).await?;

    Ok(HttpResponse::Ok().json(secret_keys))
}


#[delete("/delete")]
pub async fn delete_secret_key(session_user: web::ReqData<SessionUser>, query: web::Query<SkDeleteDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let sk_query_params = query.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:keys:view", "vault:keys:delete"])?;
    session_user.has_tag_permission(&state, sk_query_params.tag_id).await?;

    // Delete the secret key using the provided data and the session user's context
    let result = vault_secret_keys::delete_secret_key_and_versions(
        &state.pg_pool,
        session_user.organization_id,
        sk_query_params.project_id,
        sk_query_params.tag_id,
        sk_query_params.secret_id,
    ).await?;
    if result == 0 {
        return Ok(HttpResponse::BadRequest().body("Secret key not found, or not accessible to the user"));
    }

    Ok(HttpResponse::NoContent().finish())
}
