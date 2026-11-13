-- Organization to manage different organizations within N-Ops
CREATE TABLE organization (
    organization_id UUID PRIMARY KEY,
    organization_name VARCHAR(250) UNIQUE NOT NULL, -- Name of the organization (Unique across all organizations)

    -- Metadata about the organization (not indexed)
    organization_info JSONB NOT NULL,
    -- billing_info JSONB NOT NULL,  -- Billing and GST, SLA and other financial details
    encryption_key TEXT NOT NULL,  -- AES-256-GCM encryption key (Encrypted with Master Key from environment)

    -- Quota for the organization
    allocated_ai_credit BIGINT NOT NULL,  -- Total AI credits allocated to the organization
    utilized_ai_credit BIGINT NOT NULL,  -- AI credits utilized by the organization

    allocated_storage_bytes BIGINT NOT NULL,  -- Total storage allocated to the organization
    utilized_storage_bytes BIGINT NOT NULL,  -- Storage utilized by the organization

    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    -- Limits for the organization
    user_limit INT NOT NULL,     -- Total number of users allowed in the organization
    server_limit INT NOT NULL,   -- Total number of servers allowed in the organization
    workflow_limit INT NOT NULL,   -- Total number of workflows allowed in the organization

    -- Limits for secret keys/stores within the organization (SS - Secret Store)
    ss_project_limit INT NOT NULL,   -- Total number of projects allowed in the organization
    ss_keys_limit INT NOT NULL,      -- Total number of secret keys allowed per project
    ss_notes_limit INT NOT NULL,   -- Total number of notes allowed per project

    -- Additional limits can be added here as needed

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);


-- Tag for categorizing resources within an organization
CREATE TABLE resource_tag (
    tag_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    tag_name VARCHAR(50) NOT NULL,
    tag_description TEXT NOT NULL,

    UNIQUE (tag_name, organization_id)  -- Unique tag name per organization
);


-- IP based Geographic Locations (IP range to Country mapping)
CREATE TABLE ip_geo_locations (
    network CIDR PRIMARY KEY,
    country_iso_code TEXT NOT NULL,
    country_name TEXT NOT NULL
);
