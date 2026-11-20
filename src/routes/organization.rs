use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::organization::{
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
