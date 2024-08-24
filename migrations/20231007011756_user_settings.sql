CREATE SEQUENCE user_settings_index_seq;

CREATE TABLE IF NOT EXISTS user_settings
(
    index                            numeric(20, 0) not null default nextval('user_settings_index_seq') constraint chk_index_range check (index >= 0 and index <= 18446744073709551615) primary key,
    afk_timeout                      int            null,
    allow_accessibility_detection    boolean        null,
    animate_emoji                    boolean        null,
    animate_stickers                 int            null,
    contact_sync_enabled             boolean        null,
    convert_emoticons                boolean        null,
    custom_status                    text           null,
    default_guilds_restricted        boolean        null,
    detect_platform_accounts         boolean        null,
    developer_mode                   boolean        null,
    disable_games_tab                boolean        null,
    enable_tts_command               boolean        null,
    explicit_content_filter          int            null,
    friend_source_flags              text           null,
    gateway_connected                boolean        null,
    gif_auto_play                    boolean        null,
    guild_folders                    text           null,
    guild_positions                  text           null,
    inline_attachment_media          boolean        null,
    inline_embed_media               boolean        null,
    locale                           varchar(255)   null,
    message_display_compact          boolean        null,
    native_phone_integration_enabled boolean        null,
    render_embeds                    boolean        null,
    render_reactions                 boolean        null,
    restricted_guilds                text           null,
    show_current_game                boolean        null,
    status                           varchar(255)   null,
    stream_notifications_enabled     boolean        null,
    theme                            varchar(255)   null,
    timezone_offset                  int            null
);

