# Permissions

All permissions are listed below with their corresponding descriptions

Update all user roles with given permissions

```sql
UPDATE user_role
SET permissions = ARRAY(
    SELECT DISTINCT permission
    FROM UNNEST(
        permissions || ARRAY[
            'vault:project:view',
            'vault:project:create',
            'vault:project:edit',
            'vault:project:delete'
        ]::TEXT[]
    ) AS permission
),
updated_at = NOW();
```

## Tag Permissions

Tags permissions are related to managing and accessing tags within the system.

We do not recommend generating more than 25 tags, as it may become difficult to manage and maintain them effectively.

| Permission | Description |
|------------|-------------|
| tag:create | Allows creation of new tags |
| tag:edit   | Allows editing of existing tags |
| tag:delete | Allows deletion of tags |


## User Role Permissions

User role permissions are related to managing and accessing user roles within the system.

| Permission | Description |
|------------|-------------|
| user:role:view | Allows viewing of user roles |
| user:role:create | Allows creation of new user roles |
| user:role:edit   | Allows editing of existing user roles |
| user:role:delete | Allows deletion of user roles |


## Vault - Project

Vault project permissions are related to managing and accessing projects within the vault.

| Permission | Description |
|------------|-------------|
| vault:project:view | Allows viewing of vault projects |
| vault:project:create | Allows creation of new vault projects |
| vault:project:edit   | Allows editing of existing vault projects |
| vault:project:delete | Allows deletion of vault projects |

