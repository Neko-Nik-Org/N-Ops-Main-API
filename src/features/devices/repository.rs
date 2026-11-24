use super::dto::CreateDeviceSessionDto;
use deadpool_postgres::Pool as PgPool;
use super::model::DeviceSession;
use crate::errors::PgResult;



pub async fn create_new_device_session(db_pool: &PgPool, user_id: i64, session_data: &CreateDeviceSessionDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO device_session (
                user_id,
                device_name,
                device_type,
                device_identifier,
                device_details,
                session_token_hash,
                fcm_token,
                apns_token
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            &[
                &user_id,
                &session_data.device_name,
                &session_data.device_type,
                &session_data.device_identifier,
                &session_data.device_details,
                &session_data.session_token_hash,
                &session_data.fcm_token,
                &session_data.apns_token,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_device(db_pool: &PgPool, user_id: i64, session_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM device_session
            WHERE user_id = $1 AND session_id = $2
            "#,
            &[&user_id, &session_id],
        )
        .await?;

    Ok(result)
}


pub async fn get_device(db_pool: &PgPool, user_id: i64, session_id: i64) -> PgResult<Option<DeviceSession>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                session_id,
                device_name,
                device_type,
                device_identifier,
                device_details,
                created_at,
                last_accessed_at
            FROM device_session
            WHERE user_id = $1 AND session_id = $2
            "#,
            &[&user_id, &session_id],
        )
        .await?;

    Ok(row.map(DeviceSession::from))
}


pub async fn list_all_devices(db_pool: &PgPool, user_id: i64) -> PgResult<Vec<DeviceSession>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                session_id,
                device_name,
                device_type,
                device_identifier,
                device_details,
                created_at,
                last_accessed_at
            FROM device_session
            WHERE user_id = $1
            "#,
            &[&user_id],
        )
        .await?;

    Ok(DeviceSession::from_rows(rows))
}


// pub async fn update_device_tokens
// pub async fn update_last_accessed_at
