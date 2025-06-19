CREATE SEQUENCE user_settings_index_seq;

CREATE TABLE IF NOT EXISTS user_settings
(
    index numeric(20, 0) NOT NULL DEFAULT nextval(
        'user_settings_index_seq'
    ) CONSTRAINT chk_index_range CHECK (
        index >= 0 AND index <= 18446744073709551615
    ) PRIMARY KEY,
    afk_timeout int NULL,
    allow_accessibility_detection boolean NULL,
    animate_emoji boolean NULL,
    animate_stickers int NULL,
    contact_sync_enabled boolean NULL,
    convert_emoticons boolean NULL,
    custom_status text NULL,
    default_guilds_restricted boolean NULL,
    detect_platform_accounts boolean NULL,
    developer_mode boolean NULL,
    disable_games_tab boolean NULL,
    enable_tts_command boolean NULL,
    explicit_content_filter int NULL,
    friend_source_flags text NULL,
    gateway_connected boolean NULL,
    gif_auto_play boolean NULL,
    guild_folders text NULL,
    guild_positions text NULL,
    inline_attachment_media boolean NULL,
    inline_embed_media boolean NULL,
    locale varchar(255) NULL,
    message_display_compact boolean NULL,
    native_phone_integration_enabled boolean NULL,
    render_embeds boolean NULL,
    render_reactions boolean NULL,
    restricted_guilds text NULL,
    show_current_game boolean NULL,
    status varchar(255) NULL,
    stream_notifications_enabled boolean NULL,
    theme varchar(255) NULL,
    timezone_offset int NULL
);
