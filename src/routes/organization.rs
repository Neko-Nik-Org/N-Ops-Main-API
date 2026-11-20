use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::organization::{
    CreateOrgDto,
    ListOrgsDto,
    self
};



#[get("/get/{organization_id}")]
pub async fn get_organization(path: web::Path<uuid::Uuid>, state: web::Data<AppState>) -> ApiResponse {
    let organization_id = path.into_inner();

    // Get the organization using the provided organization_id
    let organization = organization::get_organization(&state.pg_pool, organization_id).await?;

    Ok(HttpResponse::Ok().json(organization))
}


#[get("/list")]
pub async fn list_organizations(query: web::Query<ListOrgsDto>, state: web::Data<AppState>) -> ApiResponse {
    let list_orgs_query = query.into_inner();

    // List organizations based on the provided query parameters
    let organizations = organization::list_organizations(&state.pg_pool, list_orgs_query).await?;

    Ok(HttpResponse::Ok().json(organizations))
}


#[post("/create")]
pub async fn create_organization(body: web::Json<CreateOrgDto>, state: web::Data<AppState>) -> ApiResponse {
    let org_data = body.into_inner();

    // Create a new organization using the provided data
    organization::create_organization(&state, &org_data).await?;

    Ok(HttpResponse::Created().finish())
}


#[delete("/delete/{organization_id}")]
pub async fn delete_organization(path: web::Path<uuid::Uuid>, state: web::Data<AppState>) -> ApiResponse {
    let organization_id = path.into_inner();

    // Delete the organization using the provided organization_id
    organization::delete_organization(&state.pg_pool, organization_id).await?;

    Ok(HttpResponse::NoContent().finish())
}


#[put("/state/{organization_id}/{is_active}")]
pub async fn organization_state_change(path: web::Path<(uuid::Uuid, bool)>, state: web::Data<AppState>) -> ApiResponse {
    let (organization_id, is_active) = path.into_inner();

    // Change the state of the organization using the provided organization_id and is_active flag
    organization::organization_state_change(&state.pg_pool, organization_id, is_active).await?;

    Ok(HttpResponse::Ok().finish())
}
