-- API keys for an organization
CREATE TABLE api_key (
    api_key_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- Lookup ID for the API key
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    key_name VARCHAR(100) NOT NULL,
    key_description TEXT NOT NULL,  -- Reason or description for the API key

    -- Example: 'nk-{api_key_id}_base64({actual_api_key_value})' so that its easy to do lookups and verify the key
    key_hash TEXT UNIQUE NOT NULL,  -- HMAC-SHA-256 of the API key value

    -- Only has 'Keys vault' & 'Config files' resource permissions (No other resource management allowed programmatically)
    resource_permissions TEXT[] NOT NULL,  -- Resource-specific permissions for the API key
    -- api:vault:keys:view
    -- api:vault:config:view

    -- Associated resource tag for the API key (Only one tag can be associated per API key)
    resource_tag BIGINT NOT NULL REFERENCES resource_tag(tag_id) ON DELETE CASCADE,

    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at TIMESTAMPTZ,  -- Expiration date for the API key (optional)

    UNIQUE (key_name, organization_id)  -- Unique key name per organization
);


CREATE TABLE api_key_vault_service_mapping (
    api_key_id BIGINT NOT NULL REFERENCES api_key(api_key_id) ON DELETE CASCADE,
    vault_service_id BIGINT NOT NULL REFERENCES vault_service(service_id) ON DELETE CASCADE,

    PRIMARY KEY (api_key_id, vault_service_id)
);
