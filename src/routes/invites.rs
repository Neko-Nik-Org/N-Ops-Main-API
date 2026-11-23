use actix_web::{HttpResponse, delete, get, post, web};
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


#[delete("/delete/{invitation_id}")]
pub async fn delete_invite(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let invitation_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:invite:view", "user:invite:delete"])?;

    // Delete the invite using the provided data and the session user's context
    user_invite::delete_invitation(&state.pg_pool, session_user.organization_id, invitation_id).await?;

    Ok(HttpResponse::NoContent().finish())
}
