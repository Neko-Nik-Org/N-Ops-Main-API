use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::ar_policy::{
    CreateResourceTagDto,
    EditResourceTagDto,
    self
};



#[get("/get/{policy_id}")]
pub async fn fetch_policy(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let policy_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["policy:ar:view"])?;

    // Get the tag using the provided data and the session user's context
    let policy_data = ar_policy::get_policy(&state.pg_pool, session_user.organization_id, policy_id).await?;
    if policy_data.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().json(policy_data))
}
