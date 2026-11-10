use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::tags::{
    CreateResourceTagDto,
    EditResourceTagDto,
    self
};



#[post("/create")]
pub async fn create_tag(session_user: web::ReqData<SessionUser>, data: web::Json<CreateResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_tag_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:view", "tag:create"])?;

    // Create the new tag using the provided data and the session user's context
    tags::create_new_tag(&state, &session_user, &new_tag_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[put("/update")]
pub async fn update_tag(session_user: web::ReqData<SessionUser>, data: web::Json<EditResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_tag_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:view", "tag:edit"])?;
    session_user.has_tag_permission(new_tag_data.tag_id)?;

    // Update the tag using the provided data and the session user's context
    tags::update_tag_data(&state, &session_user, &new_tag_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[delete("/delete/{tag_id}")]
pub async fn delete_tag(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let tag_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:view", "tag:delete"])?;
    session_user.has_tag_permission(tag_id)?;

    // Delete the tag using the provided data and the session user's context
    tags::delete_tag(&state, &session_user, tag_id).await?;

    Ok(HttpResponse::Ok().finish())
}


#[get("/get/{tag_id}")]
pub async fn get_tag(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let tag_id = path.into_inner();

    // Validate that the session user has the required permissions
    // No need to check for "tag:view" permission, this endpoint will be used across all services that require tag information
    session_user.has_tag_permission(tag_id)?;

    // Get the tag using the provided data and the session user's context
    let tag = tags::get_tag(&state, &session_user, tag_id).await?;

    Ok(HttpResponse::Ok().json(tag))
}


#[get("/list")]
pub async fn list_tags(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:view"])?;

    // Get the list of all tags for the session user's organization (No tag-specific filtering applied)
    let tags = tags::list_all_tags(&state, &session_user).await?;

    Ok(HttpResponse::Ok().json(tags))
}
