use actix_web::{HttpResponse, delete, get, post, put, web};
use crate::errors::ApiResponse;
use crate::state::AppState;
use crate::features::organization::{
    UpdateOrgQuotaDto,
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
    let result = organization::delete_organization(&state.pg_pool, organization_id).await?;
    if result == 0 {
        return Ok(HttpResponse::ImUsed().finish());
    }

    Ok(HttpResponse::NoContent().finish())
}


#[put("/state/{organization_id}/{is_active}")]
pub async fn organization_state_change(path: web::Path<(uuid::Uuid, bool)>, state: web::Data<AppState>) -> ApiResponse {
    let (organization_id, is_active) = path.into_inner();

    // Change the state of the organization using the provided organization_id and is_active flag
    let result = organization::organization_state_change(&state.pg_pool, organization_id, is_active).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}


#[put("/update/billing/{organization_id}")]
pub async fn billing_info_update(path: web::Path<uuid::Uuid>, body: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    let organization_id = path.into_inner();
    let billing_info = body.into_inner();

    // Replace the billing information for the organization using the provided organization_id and billing_info
    let result = organization::update_organization_billing_info(&state.pg_pool, organization_id, &billing_info).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}


#[put("/update/quota/{organization_id}")]
pub async fn quota_update(path: web::Path<uuid::Uuid>, body: web::Json<UpdateOrgQuotaDto>, state: web::Data<AppState>) -> ApiResponse {
    let organization_id = path.into_inner();
    let quota_data = body.into_inner();

    // Update the quota for the organization using the provided organization_id and quota_data
    let result = organization::quota_update(&state.pg_pool, organization_id, &quota_data).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}


#[put("/update/basic/{organization_id}")]
pub async fn basic_info_update(path: web::Path<uuid::Uuid>, body: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    let organization_id = path.into_inner();
    let basic_info = body.into_inner();

    // Update the basic information for the organization using the provided organization_id and basic_info
    let result = organization::update_organization_info(&state.pg_pool, organization_id, &basic_info).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}


#[get("/check-name/{organization_name}")]
pub async fn check_org_name(path: web::Path<String>, state: web::Data<AppState>) -> ApiResponse {
    let organization_name = path.into_inner();

    // Check if the organization name is available
    let name_exists = organization::check_org_name(&state.pg_pool, &organization_name).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "is_available": !name_exists })))
}


#[put("/update/name/{organization_id}")]
pub async fn name_update(path: web::Path<uuid::Uuid>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    let organization_id = path.into_inner();
    let new_name = body.into_inner();

    // Update the name for the organization using the provided organization_id and new_name
    let result = organization::name_update(&state.pg_pool, organization_id, &new_name).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
