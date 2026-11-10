use actix_web::{HttpResponse, delete, post, put, web};
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

    // Delete the tag using the provided data and the session user's context
    tags::delete_tag(&state, &session_user, tag_id).await?;

    Ok(HttpResponse::Ok().finish())
}
