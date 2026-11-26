# Permissions

All permissions are listed below with their corresponding descriptions

Update all user roles with given permissions

```sql
UPDATE user_role
SET permissions = ARRAY(
    SELECT DISTINCT permission
    FROM UNNEST(
        permissions || ARRAY[
            'user:view',
            'user:create',
            'user:edit',
            'user:delete'
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


## User Permissions

User permissions are related to managing and accessing user accounts within the system.

> Note: If the user has user:create permission, they can bypass tag and role level restrictions when creating a new user. *So, make sure granting this permission is done carefully and only to trusted users!*


| Permission | Description |
|------------|-------------|
| user:view | Allows viewing of user accounts |
| user:create | Allows creation of new user accounts |
| user:edit   | Allows editing of existing user accounts |
| user:delete | Allows deletion of user accounts |


## Access Restriction Policy Permissions

Access restriction policy permissions are related to managing and accessing access restriction policies within the system.

| Permission | Description |
|------------|-------------|
| policy:ar:view | Allows viewing of access restriction policies |
| policy:ar:create | Allows creation of new access restriction policies |
| policy:ar:edit   | Allows editing of existing access restriction policies |
| policy:ar:delete | Allows deletion of access restriction policies |


## Organization Invitation Permissions

User invitation permissions are related to managing and accessing organization invitations within the system.

| Permission | Description |
|------------|-------------|
| user:invite:view | Allows viewing of organization invitations |
| user:invite:create | Allows sending/re-sending new organization invitations |
| user:invite:delete | Allows deletion of organization invitations |


## Vault - Secret Keys

Vault secret key permissions are related to managing and accessing secret keys within the vault.

| Permission | Description |
|------------|-------------|
| vault:keys:view | Allows viewing of vault secret keys |
| vault:keys:create | Allows creation of new vault secret keys |
| vault:keys:edit   | Allows editing of existing vault secret keys |
| vault:keys:delete | Allows deletion of vault secret keys |
