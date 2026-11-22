use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::ar_policy::{
    CreatePolicyDto,
    ListPolicyDto,
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


#[get("/list")]
pub async fn list_policies(session_user: web::ReqData<SessionUser>, path: web::Query<ListPolicyDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let list_policy_dto = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["policy:ar:view"])?;

    // Get the tag using the provided data and the session user's context
    let policy_data = ar_policy::list_policies(&state.pg_pool, session_user.organization_id, list_policy_dto).await?;

    Ok(HttpResponse::Ok().json(policy_data))
}


#[post("/create")]
pub async fn create_policy(session_user: web::ReqData<SessionUser>, body: web::Json<CreatePolicyDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let create_policy_dto = body.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["policy:ar:view", "policy:ar:create"])?;

    // Create the policy using the provided data and the session user's context
    let policy_data = ar_policy::create_policy(&state.pg_pool, session_user.organization_id, create_policy_dto).await?;

    Ok(HttpResponse::Ok().json(policy_data))
}


#[get("/check/{policy_id}")]
pub async fn check_policy_association(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let policy_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["policy:ar:view"])?;

    // Check the policy association (Count of users that have this policy, useful to do force delete and to know the impact)
    let association_count = ar_policy::count_policy_associations(&state.pg_pool, session_user.organization_id, policy_id).await?;

    Ok(HttpResponse::Ok().json(association_count))
}


#[delete("/delete/{policy_id}")]
pub async fn delete_policy(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let policy_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["policy:ar:view", "policy:ar:delete"])?;

    // Delete the policy using the provided data and the session user's context
    ar_policy::delete_policy(&state.pg_pool, session_user.organization_id, policy_id).await?;

    Ok(HttpResponse::NoContent().finish())
}
