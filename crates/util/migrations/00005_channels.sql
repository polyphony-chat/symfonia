CREATE TABLE IF NOT EXISTS channels (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    position integer NOT NULL,
    parent varchar(255) NOT NULL,
    parent_type entity_type NOT NULL,
    permission_overrides permission_overrides [] NULL,
    channel_data_reference bigint UNIQUE NULL,
    channel_data_encrypted text NULL,
    CONSTRAINT fk_channel_data FOREIGN KEY (
        channel_data_reference
    ) REFERENCES channel_data (id) ON DELETE CASCADE
);

-- Ensure that either encrypted data is present, or that there exists a foreign key
-- to the channel_data table.
ALTER TABLE channels
ADD CONSTRAINT either_data_reference_or_encrypted_data
CHECK (channel_data_reference IS NOT null OR channel_data_encrypted IS NOT null);
