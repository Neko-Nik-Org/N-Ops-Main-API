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
    session_user.has_all_permissions(&["tag:create"])?;

    // Create the new tag using the provided data and the session user's context
    tags::create_new_tag(&state, &session_user, &new_tag_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[put("/update")]
pub async fn update_tag(session_user: web::ReqData<SessionUser>, data: web::Json<EditResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_tag_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:edit"])?;
    session_user.has_tag_permission(&state, new_tag_data.tag_id).await?;

    // Update the tag using the provided data and the session user's context
    tags::update_tag_data(&state, &session_user, &new_tag_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[delete("/delete/{tag_id}")]
pub async fn delete_tag(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let tag_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:delete"])?;
    session_user.has_tag_permission(&state, tag_id).await?;

    // Delete the tag using the provided data and the session user's context
    tags::delete_tag(&state, &session_user, tag_id).await?;

    Ok(HttpResponse::Ok().finish())
}


#[get("/get/{tag_id}")]
pub async fn get_tag(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let tag_id = path.into_inner();

    // Validate that the session user has the required permissions
    session_user.has_tag_permission(&state, tag_id).await?;

    // Get the tag using the provided data and the session user's context
    let tag = tags::get_tag(&state, &session_user, tag_id).await?;

    Ok(HttpResponse::Ok().json(tag))
}


#[get("/list")]
pub async fn list_tags(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // TODO: No need to do permission check, since the same endpoint will be used everywhere to list tags
    // - Cache the tags
    // - Write a service function to do all the below logic and return the accessible tags directly

    // Get the list of all tags for the session user's organization (No tag-specific filtering applied)
    let tags = tags::list_all_tags(&state, &session_user).await?;

    // List only the tags that the session user has permission to view
    let tag_ids = tags.iter().map(|tag| tag.tag_id).collect::<Vec<_>>();
    let accessible_tag_ids = session_user.accessible_tags(&tag_ids);

    // If there are no accessible tags, return all tags (means the user has access to all tags by default)
    if accessible_tag_ids.is_empty() {
        return Ok(HttpResponse::Ok().json(tags));
    }

    let final_tags = tags.into_iter()
        .filter(|tag| accessible_tag_ids.contains(&tag.tag_id))
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(final_tags))
}
