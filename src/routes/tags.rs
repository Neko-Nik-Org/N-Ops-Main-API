use actix_web::{put, post, web, HttpResponse};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::tags::{
    CreateResourceTagDto,
    EditResourceTagDto,
    create_new_tag,
    update_tag_data
};



#[post("/create")]
pub async fn create_tag(session_user: web::ReqData<SessionUser>, data: web::Json<CreateResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_tag_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:view", "tag:create"])?;

    // Create the new tag using the provided data and the session user's context
    create_new_tag(&state, &session_user, &new_tag_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[put("/update")]
pub async fn update_tag(session_user: web::ReqData<SessionUser>, data: web::Json<EditResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let new_tag_data = data.into_inner(); 

    // Validate that the session user has the required permissions
    session_user.has_all_permissions(&["tag:view", "tag:edit"])?;

    // Update the tag using the provided data and the session user's context
    update_tag_data(&state, &session_user, &new_tag_data).await?;

    Ok(HttpResponse::Created().finish())
}
