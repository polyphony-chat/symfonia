CREATE SEQUENCE user_settings_index_seq;

CREATE TABLE IF NOT EXISTS user_settings
(
    index                            numeric(20, 0) not null default nextval('user_settings_index_seq') constraint chk_index_range check (index >= 0 and index <= 18446744073709551615) primary key,
    afk_timeout                      int            null,
    allow_accessibility_detection    smallint       null,
    animate_emoji                    smallint       null,
    animate_stickers                 int            null,
    contact_sync_enabled             smallint       null,
    convert_emoticons                smallint       null,
    custom_status                    text           null,
    default_guilds_restricted        smallint       null,
    detect_platform_accounts         smallint       null,
    developer_mode                   smallint       null,
    disable_games_tab                smallint       null,
    enable_tts_command               smallint       null,
    explicit_content_filter          int            null,
    friend_source_flags              text           null,
    gateway_connected                smallint       null,
    gif_auto_play                    smallint       null,
    guild_folders                    text           null,
    guild_positions                  text           null,
    inline_attachment_media          smallint       null,
    inline_embed_media               smallint       null,
    locale                           varchar(255)   null,
    message_display_compact          smallint       null,
    native_phone_integration_enabled smallint       null,
    render_embeds                    smallint       null,
    render_reactions                 smallint       null,
    restricted_guilds                text           null,
    show_current_game                smallint       null,
    status                           varchar(255)   null,
    stream_notifications_enabled     smallint       null,
    theme                            varchar(255)   null,
    timezone_offset                  int            null
);

