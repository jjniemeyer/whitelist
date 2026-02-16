CREATE TYPE booking_status AS ENUM ('pending', 'approved', 'denied');

CREATE TABLE IF NOT EXISTS bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    caller_name VARCHAR(255) NOT NULL,
    caller_phone VARCHAR(20) NOT NULL,
    caller_email VARCHAR(255),
    call_reason TEXT,
    status booking_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    whitelist_entry_id UUID REFERENCES whitelist_entries(id)
);

CREATE INDEX idx_bookings_status ON bookings(status, created_at DESC);
CREATE INDEX idx_bookings_phone ON bookings(caller_phone);
