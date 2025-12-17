-- Supabase migration: Create saved_addresses table
-- This table stores user shipping addresses identified by SSH key fingerprint

CREATE TABLE IF NOT EXISTS saved_addresses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_fingerprint TEXT NOT NULL,
    name TEXT NOT NULL,
    street_1 TEXT NOT NULL,
    street_2 TEXT DEFAULT '',
    city TEXT NOT NULL,
    state TEXT DEFAULT '',
    country TEXT NOT NULL,
    phone TEXT DEFAULT '',
    postal_code TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for faster lookups by user fingerprint
CREATE INDEX IF NOT EXISTS idx_saved_addresses_user_fingerprint 
ON saved_addresses(user_fingerprint);

-- Comment on table
COMMENT ON TABLE saved_addresses IS 'Stores user shipping addresses, identified by SSH key fingerprint';
COMMENT ON COLUMN saved_addresses.user_fingerprint IS 'SHA256 hash of user SSH public key';

-- Enable Row Level Security (optional - uncomment if needed)
-- Note: For anonymous auth with anon key, RLS policies are optional
-- If you enable RLS, use these policies:

-- ALTER TABLE saved_addresses ENABLE ROW LEVEL SECURITY;

-- DROP POLICY IF EXISTS "Allow read by fingerprint" ON saved_addresses;
-- CREATE POLICY "Allow read by fingerprint" ON saved_addresses FOR SELECT USING (true);

-- DROP POLICY IF EXISTS "Allow insert addresses" ON saved_addresses;
-- CREATE POLICY "Allow insert addresses" ON saved_addresses FOR INSERT WITH CHECK (true);

-- DROP POLICY IF EXISTS "Allow delete addresses" ON saved_addresses;
-- CREATE POLICY "Allow delete addresses" ON saved_addresses FOR DELETE USING (true);

