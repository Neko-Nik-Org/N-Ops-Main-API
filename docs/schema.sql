-- SQL schema for N-Ops


-- Organization to manage different organizations within N-Ops
CREATE TABLE organization (
    organization_id UUID PRIMARY KEY,
    organization_name VARCHAR(250) UNIQUE NOT NULL, -- Name of the organization (Unique across all organizations)

    -- Metadata about the organization (not indexed)
    organization_info JSONB NOT NULL,

    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    -- Limits for the organization
    user_limit INT,     -- Total number of users allowed in the organization
    server_limit INT,   -- Total number of servers allowed in the organization

    -- Limits for secret keys/stores within the organization (SS - Secret Store)
    ss_project_limit INT,   -- Total number of projects allowed in the organization
    ss_keys_limit INT,      -- Total number of secret keys allowed per project

    -- Additional limits can be added here as needed

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);


-- Application users within an organization
CREATE TABLE app_user (
    user_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_name VARCHAR(100) NOT NULL UNIQUE,  -- e.g., 'john_doe'
    primary_email VARCHAR(254) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,

    password_hash TEXT NOT NULL,    -- Bcrypt hash of the user's password
    user_details JSONB NOT NULL,  -- Just for storing extra info (not indexed)
    ui_info JSONB NOT NULL,  -- e.g., theme, state, etc. (not indexed)

    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    is_email_verified BOOLEAN DEFAULT FALSE NOT NULL,  -- Is email verified

    -- Role assigned to the user within the organization
    role_id BIGINT NOT NULL REFERENCES user_role(role_id) ON DELETE CASCADE,

    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,
    has_totp_2fa BOOLEAN DEFAULT FALSE NOT NULL,  -- Is TOTP 2FA enabled for this user
    has_email_2fa BOOLEAN DEFAULT FALSE NOT NULL,  -- Is Email 2FA enabled for this user
    -- Other 2FA methods (Can use same table structure to manage WebAuthn/FIDO2)
    -- WebAuthn -> Touch ID, Face ID, Windows Hello, Android/iOS Biometrics
    -- FIDO2 -> Security keys like YubiKey, etc.

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    UNIQUE (display_name, organization_id)  -- Unique display name per organization
);


-- Roles within an organization
CREATE TABLE user_role (
    role_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    role_name VARCHAR(100) NOT NULL,
    role_description TEXT NOT NULL,

    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    permissions TEXT[] NOT NULL,  -- e.g., ['user:create', 'user:delete']

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    UNIQUE (role_name, organization_id)  -- Unique role name per organization
);
