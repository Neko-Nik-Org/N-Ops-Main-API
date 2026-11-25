use super::dto::{DeviceLoginReqDto, DeviceSessionDto};
use deadpool_postgres::Pool as PgPool;
use super::model::DeviceSession;
use crate::errors::PgResult;



pub async fn create_new_device_session(db_pool: &PgPool, login_req: &DeviceLoginReqDto, session_token_hash: &str) -> PgResult<u64> {
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
                session_token_hash
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            &[
                &login_req.user_id,
                &login_req.device_name,
                &login_req.device_type,
                &login_req.device_identifier,
                &login_req.device_details,
                &session_token_hash,
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


pub async fn get_device_session(db_pool: &PgPool, user_id: i64, device_identifier: &str) -> PgResult<Option<DeviceSessionDto>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                ds.session_id AS device_session_id,
                u.user_name,
                ds.session_token_hash
            FROM device_session ds
            INNER JOIN app_user u
                ON u.user_id = ds.user_id
            INNER JOIN organization o
                ON o.organization_id = u.organization_id
            LEFT JOIN access_restriction_policy p
                ON p.policy_id = u.access_restriction_policy_id
            WHERE
                ds.user_id = $1
                AND ds.device_identifier = $2
                AND ds.last_accessed_at >= NOW() - INTERVAL '7 days'
                AND u.is_active = TRUE
                AND o.is_active = TRUE
                AND COALESCE(p.allow_device_logins, TRUE) = TRUE
            "#,
            &[&user_id, &device_identifier],
        )
        .await?;

    Ok(row.map(DeviceSessionDto::from))
}


pub async fn update_fcm_token(db_pool: &PgPool, user_id: i64, session_id: i64, fcm_token: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE device_session
            SET fcm_token = $1
            WHERE user_id = $2 AND session_id = $3
            "#,
            &[&fcm_token, &user_id, &session_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_apns_token(db_pool: &PgPool, user_id: i64, session_id: i64, apns_token: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE device_session
            SET apns_token = $1
            WHERE user_id = $2 AND session_id = $3
            "#,
            &[&apns_token, &user_id, &session_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_last_accessed_at(db_pool: &PgPool, user_id: i64, session_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE device_session
            SET last_accessed_at = NOW()
            WHERE user_id = $1 AND session_id = $2
            "#,
            &[&user_id, &session_id],
        )
        .await?;

    Ok(result)
}
