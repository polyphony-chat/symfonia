CREATE TABLE IF NOT EXISTS embeds ();
CREATE TABLE IF NOT EXISTS reactions ();

CREATE TABLE IF NOT EXISTS message_data (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NULL,
    -- TODO: Make these proper types/fk's
    attachments TEXT [] NULL,
    embeds TEXT [] NULL,
    reactions TEXT [] NULL
);

CREATE TABLE IF NOT EXISTS messages (
    version NUMERIC(20, 0) NOT NULL CONSTRAINT chk_range CHECK (version >= 0 AND version <= 18446744073709551615),
    uid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    author TEXT NOT NULL, -- TODO: fk constraint
    timestamp TIMESTAMP
);
