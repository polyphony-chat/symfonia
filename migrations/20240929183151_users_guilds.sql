CREATE TABLE IF NOT EXISTS users_guilds (
    user_id NUMERIC(20, 0) NOT NULL,
    guild_id NUMERIC(20, 0) NOT NULL,
    PRIMARY KEY (user_id, guild_id),
    CONSTRAINT fk_users_guilds_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_users_guilds_guild_id FOREIGN KEY (guild_id) REFERENCES guilds (id) ON DELETE CASCADE
);