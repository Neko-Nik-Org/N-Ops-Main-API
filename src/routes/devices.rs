use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::devices::{
    CreateDeviceSessionDto,
    self
};



// #[post("/create")]
// pub async fn create_tag(session_user: web::ReqData<SessionUser>, data: web::Json<CreateResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let new_tag_data = data.into_inner(); 

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["tag:create"])?;

//     // Create the new tag using the provided data and the session user's context
//     tags::create_new_tag(&state, &session_user, &new_tag_data).await?;

//     Ok(HttpResponse::Created().finish())
// }


// #[put("/update")]
// pub async fn update_tag(session_user: web::ReqData<SessionUser>, data: web::Json<EditResourceTagDto>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let new_tag_data = data.into_inner(); 

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["tag:edit"])?;
//     session_user.has_tag_permission(new_tag_data.tag_id)?;

//     // Update the tag using the provided data and the session user's context
//     tags::update_tag_data(&state, &session_user, &new_tag_data).await?;

//     Ok(HttpResponse::Created().finish())
// }


// #[delete("/delete/{tag_id}")]
// pub async fn delete_tag(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
//     let session_user = session_user.into_inner();
//     let tag_id = path.into_inner();

//     // Validate that the session user has the required permissions
//     session_user.has_all_permissions(&["tag:delete"])?;
//     session_user.has_tag_permission(tag_id)?;

//     // Delete the tag using the provided data and the session user's context
//     tags::delete_tag(&state, &session_user, tag_id).await?;

//     Ok(HttpResponse::Ok().finish())
// }


#[get("/get/{session_id}")]
pub async fn get_device(session_user: web::ReqData<SessionUser>, path: web::Path<i64>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let session_id = path.into_inner();

    // Get the device using the provided data and the session user's context
    let device = devices::get_device(&state.pg_pool, session_user.user_id, session_id).await?;

    Ok(HttpResponse::Ok().json(device))
}


#[get("/list")]
pub async fn list_all_devices(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();

    // Note: How many devices one user can have?
    // Max 20 or even 50 !? So, no need pagination or complex filtering for now

    // Get the list of all devices for the session user's organization (No device-specific filtering applied)
    let devices = devices::list_all_devices(&state.pg_pool, session_user.user_id).await?;

    Ok(HttpResponse::Ok().json(devices))
}
