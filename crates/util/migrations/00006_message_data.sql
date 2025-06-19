CREATE TABLE IF NOT EXISTS message_data (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NULL,
    -- TODO: Make these proper types/fk's
    attachments TEXT [] NULL,
    embeds TEXT [] NULL,
    reactions TEXT [] NULL
);
