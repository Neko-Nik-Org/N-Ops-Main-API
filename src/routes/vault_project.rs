use actix_web::{HttpResponse, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::vault_projects::{
    VaultProjectDto,
    ListQueryDto,
    self
};



#[post("/create")]
pub async fn create_project(session_user: web::ReqData<SessionUser>, data: web::Json<VaultProjectDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_project_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:project:view", "vault:project:create"])?;

    // Create the new role using the provided data and the session user's context
    vault_projects::create_new_project(&state, &session_user, &new_project_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[put("/update/{project_id}")]
pub async fn update_project(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, data: web::Json<VaultProjectDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_project_data = data.into_inner();
    let project_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:project:view", "vault:project:edit"])?;

    // Update the project using the provided data and the session user's context
    vault_projects::update_project(&state, &session_user, project_id, &new_project_data).await?;

    Ok(HttpResponse::Ok().finish())
}


#[get("/list")]
pub async fn list_projects(session_user: web::ReqData<SessionUser>, query: web::Query<ListQueryDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let query = query.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:project:view"])?;

    // Fetch the list of projects using the provided query and the session user's context
    let projects = vault_projects::list_projects(&state, &session_user, &query).await?;

    Ok(HttpResponse::Ok().json(projects))
}


#[get("/detail/{project_id}")]
pub async fn get_project(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let project_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["vault:project:view"])?;

    // Fetch the project detail using the provided project ID and the session user's context
    let project = vault_projects::get_project_detail(&state, &session_user, project_id).await?;

    Ok(HttpResponse::Ok().json(project))
}
