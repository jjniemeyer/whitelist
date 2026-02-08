-- Whitelist entries table
CREATE TABLE IF NOT EXISTS whitelist_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone_number VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    is_permanent BOOLEAN NOT NULL DEFAULT false
);

-- Index for phone number lookups
CREATE INDEX idx_whitelist_phone ON whitelist_entries(phone_number);
