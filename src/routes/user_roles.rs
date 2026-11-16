use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::user_roles::{
    CreateNewRoleDto,
    self
};



#[post("/create")]
pub async fn create_role(session_user: web::ReqData<SessionUser>, data: web::Json<CreateNewRoleDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_role_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:role:view", "user:role:create"])?;

    // Create the new role using the provided data and the session user's context
    user_roles::create_new_role(&state, &session_user, &new_role_data).await?;

    Ok(HttpResponse::Created().finish())
}
