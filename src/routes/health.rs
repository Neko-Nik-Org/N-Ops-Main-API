use crate::database::health_check as pg_health_check;
use crate::errors::{ApiResponse, ServiceError};
use actix_web::{get, web, HttpResponse};
use crate::cache::redis_cache;
use crate::state::AppState;



// Health check endpoint
#[get("/service")]
async fn full_health_check(state: web::Data<AppState>) -> ApiResponse {
    // PGSQL DB Health Check
    if let Err(_) = pg_health_check(&state.pg_pool).await {
        return Err(ServiceError::PreconditionFailed("Database is not healthy!".into()));
    }

    // Redis cache health check
    if let Err(_) = redis_cache::redis_health_check(&state.redis_cache).await {
        return Err(ServiceError::PreconditionFailed("Redis cache is not healthy!".into()));
    }

    Ok(HttpResponse::Ok().body("All services are running!"))
}
