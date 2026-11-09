use deadpool_postgres::Pool as PgPool;
use super::dto::AuthInfoDbDto;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn get_user_auth_info(db_pool: &PgPool, user_name: &str) -> PgResult<Option<AuthInfoDbDto>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                -- Identifiers
                u.user_id,
                u.organization_id,

                -- Security and access information
                u.password_hash,
                o.encryption_key,
                u.has_ip_restriction,
                u.has_geo_restriction,
                u.allowed_ips,
                u.allowed_geo_locations,

                r.permissions,

                -- Role-associated resource tags
                COALESCE(
                    (
                        SELECT ARRAY_AGG(t.tag_name ORDER BY t.tag_name)
                        FROM role_tag rt
                        INNER JOIN resource_tag t
                            ON t.tag_id = rt.tag_id
                            AND t.organization_id = u.organization_id
                        WHERE rt.role_id = r.role_id
                    ),
                    ARRAY[]::TEXT[]
                ) AS tags,

                -- Basic user information
                u.user_name,
                u.display_name,
                o.organization_name,

                -- Contact information
                u.primary_email,
                u.is_email_verified,

                -- Organization limits
                o.user_limit,
                o.server_limit,
                o.ss_project_limit,
                o.ss_keys_limit,
                o.ss_file_limit,
                o.user_notes_limit,

                -- Authentication metadata
                u.last_login_at

            FROM app_user u

            INNER JOIN organization o
                ON o.organization_id = u.organization_id

            INNER JOIN user_role r
                ON r.role_id = u.role_id
                AND r.organization_id = u.organization_id

            WHERE
                u.user_name = $1
                AND u.is_active = TRUE
                AND o.is_active = TRUE
            "#,
            &[&user_name],
        )
        .await?;

    Ok(row.map(AuthInfoDbDto::from))
}
