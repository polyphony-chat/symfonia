CREATE TABLE IF NOT EXISTS channels (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    position integer NOT NULL,
    parent varchar(255) NOT NULL,
    parent_type entity_type NOT NULL,
    permission_overrides permission_overrides [] NULL,
    channel_data bigint UNIQUE NULL,
    CONSTRAINT fk_channel_data FOREIGN KEY (channel_data) references channel_data (id) ON DELETE CASCADE
);