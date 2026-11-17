use actix_web::{HttpResponse, delete, get, post, put, patch, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::user_roles::{
    UpdateRolePermissionsDto,
    UpdateRoleInfoDto,
    CreateNewRoleDto,
    ListRolesDto,
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


#[get("/list")]
pub async fn list_roles(session_user: web::ReqData<SessionUser>, query: web::Query<ListRolesDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let list_roles_query = query.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:role:view"])?;

    // List All User Roles
    // Note: No need to check any tags and permissions. Since its all Read-Only
    // Let user see all the roles without any restrictions
    let roles = user_roles::list_roles(&state.pg_pool, session_user.organization_id, &list_roles_query).await?;

    Ok(HttpResponse::Ok().json(roles))
}


#[get("/fetch/{role_id}")]
pub async fn fetch_role(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let role_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:role:view"])?;

    // Fetch the role using the provided role_id and the session user's context
    let role = user_roles::fetch_role(&state.pg_pool, session_user.organization_id, role_id).await?;

    Ok(HttpResponse::Ok().json(role))
}


#[delete("/delete/{role_id}")]
pub async fn delete_role(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let role_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:role:view", "user:role:delete"])?;

    // Call the repository function to delete the role
    user_roles::delete_role(&state, &session_user, role_id).await?;

    Ok(HttpResponse::NoContent().finish())
}


#[patch("/update/info/{role_id}")]
pub async fn update_role_info(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, data: web::Json<UpdateRoleInfoDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let role_id = path.into_inner();
    let update_data = data.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:role:view", "user:role:update"])?;

    // Call the repository function to update the role info
    user_roles::update_role_info(&state, &session_user, role_id, &update_data).await?;

    Ok(HttpResponse::Ok().finish())
}


#[put("/update/permissions/{role_id}")]
pub async fn update_role_permissions(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, data: web::Json<UpdateRolePermissionsDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let role_id = path.into_inner();
    let update_data = data.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["user:role:view", "user:role:update"])?;

    // Call the repository function to update the role permissions
    user_roles::update_role_permissions(&state, &session_user, role_id, &update_data).await?;

    Ok(HttpResponse::Ok().finish())
}
