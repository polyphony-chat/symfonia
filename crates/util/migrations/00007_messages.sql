CREATE TABLE IF NOT EXISTS messages (
    version numeric(20, 0) NOT NULL CONSTRAINT chk_range CHECK (version >= 0 AND version <= 18446744073709551615),
    uid uuid UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    author text NOT NULL,
    timestamp timestamp
);
