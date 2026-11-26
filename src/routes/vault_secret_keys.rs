use crate::features::vault_secret_keys::{self, SkNewDto};
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
    session_user.has_tag_permission(new_sk_data.tag_id)?;

    // Create the new secret key using the provided data and the session user's context
    vault_secret_keys::create_new_sk(&state, &session_user, &new_sk_data).await?;

    Ok(HttpResponse::Created().finish())
}
