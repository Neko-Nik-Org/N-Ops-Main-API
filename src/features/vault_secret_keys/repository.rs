use crate::errors::{PgError, PgPool, PgResult};
use super::dto::SkNewDto;
use uuid::Uuid;



pub async fn create_new_secret_key(db_pool: &PgPool, organization_id: Uuid, secret_key_data: &SkNewDto, encrypted_secret_value: &str) -> PgResult<u64> {
    let mut client = db_pool.get().await?;
    let transaction = client.transaction().await?;

    // 1. Lock organization and retrieve secret-key quota
    let org = transaction
        .query_opt(
            r#"
            SELECT ss_keys_limit
            FROM organization
            WHERE organization_id = $1
            FOR UPDATE
            "#,
            &[&organization_id],
        )
        .await?;

    let Some(org) = org else {
        return Err(PgError::Custom("Organization not found".into()));
    };

    let keys_limit: i32 = org.get("ss_keys_limit");

    if keys_limit < 0 {
        return Err(PgError::Custom(
            "Invalid organization secret-key quota".into(),
        ));
    }

    // 2. Validate project ownership.
    let project = transaction
        .query_opt(
            r#"
            SELECT project_id
            FROM vault_project
            WHERE project_id = $1
              AND organization_id = $2
            FOR UPDATE
            "#,
            &[
                &secret_key_data.project_id,
                &organization_id,
            ],
        )
        .await?;

    if project.is_none() {
        return Err(PgError::Custom(
            "Project not found or does not belong to this organization".into(),
        ));
    }

    // 3. Validate resource tag ownership.
    let tag = transaction
        .query_opt(
            r#"
            SELECT tag_id
            FROM resource_tag
            WHERE tag_id = $1
              AND organization_id = $2
            FOR KEY SHARE
            "#,
            &[
                &secret_key_data.tag_id,
                &organization_id,
            ],
        )
        .await?;

    if tag.is_none() {
        return Err(PgError::Custom(
            "Resource tag not found or does not belong to this organization".into(),
        ));
    }

    // 4. Count existing secret keys in the project.
    let current_keys: i64 = transaction
        .query_one(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM vault_secret_keys
            WHERE project_id = $1
            "#,
            &[&secret_key_data.project_id],
        )
        .await?
        .get(0);

    // 5. Enforce project secret-key quota.
    if current_keys >= i64::from(keys_limit) {
        return Err(PgError::Custom(
            "Project secret-key limit reached".into(),
        ));
    }

    // 6. Insert secret-key entry.
    let secret = transaction
        .query_opt(
            r#"
            INSERT INTO vault_secret_keys (
                project_id,
                secret_name,
                current_version,
                resource_tag
            )
            VALUES ($1, $2, 1, $3)
            ON CONFLICT (project_id, resource_tag, secret_name)
            DO NOTHING
            RETURNING secret_id
            "#,
            &[
                &secret_key_data.project_id,
                &secret_key_data.secret_name,
                &secret_key_data.tag_id,
            ],
        )
        .await?;

    let Some(secret) = secret else {
        return Err(PgError::Custom(
            "Secret with this name already exists for this project and tag".into(),
        ));
    };

    let secret_id: i64 = secret.get("secret_id");

    // 7. Insert initial secret-key version.
    let result = transaction
        .execute(
            r#"
            INSERT INTO vault_secret_key_versions (
                secret_id,
                version_number,
                is_hidden,
                secret_value,
                details
            )
            VALUES ($1, 1, $2, $3, $4)
            "#,
            &[
                &secret_id,
                &secret_key_data.hide_from_ui,
                &encrypted_secret_value,
                &secret_key_data.details,
            ],
        )
        .await?;

    // 9. Commit transaction.
    transaction.commit().await?;

    // 8. Return the total number of affected rows
    Ok(result)
}
