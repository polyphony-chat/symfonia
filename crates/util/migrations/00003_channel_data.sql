CREATE TABLE IF NOT EXISTS channel_data (
    id bigserial PRIMARY KEY,
    type smallint NOT NULL DEFAULT 0 CONSTRAINT chk_type CHECK (
        type >= 0
        AND type <= 3
    ),
    name varchar(64) NOT NULL,
    description text NULL
);
