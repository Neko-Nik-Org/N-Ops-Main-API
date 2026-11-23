use actix_web::{HttpResponse, delete, get, post, put, patch, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::user_invite::{
    ListInvitesDto,
    NewInviteDto,
    self
};



#[get("/list")]
pub async fn list_invites(session_user: web::ReqData<SessionUser>, path: web::Query<ListInvitesDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let list_invites_dto = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:invite:view"])?;

    // Get the list of invites using the provided data and the session user's context
    let invites_data = user_invite::list_invites(&state.pg_pool, session_user.organization_id, list_invites_dto).await?;

    Ok(HttpResponse::Ok().json(invites_data))
}


#[post("/invite")]
pub async fn new_invite(session_user: web::ReqData<SessionUser>, body: web::Json<NewInviteDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_invite_dto = body.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:invite:create", "user:create"])?;

    // Send a new invite to the specified email with the generated invitation code
    user_invite::send_new_invite(&state, &session_user, new_invite_dto).await?;

    Ok(HttpResponse::Created().finish())
}


// #[get("/check/{policy_id}")]
// pub async fn check_policy_association(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let policy_id = path.into_inner();

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["policy:ar:view"])?;

//     // Check the policy association (Count of users that have this policy, useful to do force delete and to know the impact)
//     let association_count = ar_policy::count_policy_associations(&state.pg_pool, session_user.organization_id, policy_id).await?;

//     Ok(HttpResponse::Ok().json(association_count))
// }


// #[delete("/delete/{policy_id}")]
// pub async fn delete_policy(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let policy_id = path.into_inner();

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["policy:ar:view", "policy:ar:delete"])?;

//     // Delete the policy using the provided data and the session user's context
//     ar_policy::delete_policy(&state.pg_pool, session_user.organization_id, policy_id).await?;

//     Ok(HttpResponse::NoContent().finish())
// }


// #[put("/update/{policy_id}")]
// pub async fn update_policy_info(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<UpdatePolicyBaseInfoDto>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let policy_id = path.into_inner();
//     let update_policy_dto = body.into_inner();

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["policy:ar:view", "policy:ar:edit"])?;

//     // Update the policy using the provided data and the session user's context
//     let result = ar_policy::update_policy_base_info(&state.pg_pool, session_user.organization_id, policy_id, update_policy_dto).await?;
//     if result == 0 {
//         return Ok(HttpResponse::NotAcceptable().json("Failed to update policy"));
//     }

//     Ok(HttpResponse::Ok().finish())
// }


// #[patch("/update/{policy_id}")]
// pub async fn update_policy_enforcement(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, body: web::Json<UpdatePolicyDto>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let policy_id = path.into_inner();
//     let update_full_policy_dto = body.into_inner();

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["policy:ar:view", "policy:ar:edit"])?;

//     // Update the enforcement settings of the policy
//     let result = ar_policy::update_full_policy(&state.pg_pool, session_user.organization_id, policy_id, update_full_policy_dto).await?;

//     if result == 0 {
//         return Ok(HttpResponse::NotAcceptable().json("Failed to update policy"));
//     }

//     Ok(HttpResponse::Ok().finish())
// }
