CREATE SEQUENCE members_index_seq;

CREATE TABLE IF NOT EXISTS members
(
    index numeric(20, 0) NOT NULL DEFAULT nextval(
        'members_index_seq'
    ) CONSTRAINT chk_index_range CHECK (
        index >= 0 AND index <= 18446744073709551615
    ) PRIMARY KEY,
    id numeric(20, 0) NOT NULL CONSTRAINT chk_id_range CHECK (
        id >= 0 AND id <= 18446744073709551615
    ),
    guild_id numeric(20, 0) NOT NULL CONSTRAINT chk_guild_id_range CHECK (
        guild_id >= 0 AND guild_id <= 18446744073709551615
    ),
    nick varchar(255) NULL,
    joined_at timestamp NOT NULL,
    premium_since bigint NULL,
    deaf boolean NOT NULL,
    mute boolean NOT NULL,
    pending boolean NOT NULL,
    settings text NOT NULL,
    last_message_id numeric(
        20, 0
    ) NULL CONSTRAINT chk_last_message_id_range CHECK (
        last_message_id >= 0 AND last_message_id <= 18446744073709551615
    ),
    joined_by varchar(255) NULL,
    avatar varchar(255) NULL,
    banner varchar(255) NULL,
    bio varchar(255) NOT NULL,
    theme_colors text NULL,
    pronouns varchar(255) NULL,
    communication_disabled_until timestamp NULL,
    CONSTRAINT idx_bb2bf9386ac443afbbbf9f12d3
    UNIQUE (id, guild_id),
    CONSTRAINT fk_16aceddd5b89825b8ed6029ad1c
    FOREIGN KEY (guild_id) REFERENCES guilds (id)
    ON DELETE CASCADE,
    CONSTRAINT fk_28b53062261b996d9c99fa12404
    FOREIGN KEY (id) REFERENCES users (id)
    ON DELETE CASCADE
);

ALTER SEQUENCE members_index_seq OWNED BY members.index;
