BEGIN;
INSERT INTO user_settings (
    index, afk_timeout, allow_accessibility_detection, animate_emoji, animate_stickers,
    contact_sync_enabled, convert_emoticons, custom_status, default_guilds_restricted,
    detect_platform_accounts, developer_mode, disable_games_tab, enable_tts_command,
    explicit_content_filter, friend_source_flags, gateway_connected, gif_auto_play,
    guild_folders, guild_positions, inline_attachment_media, inline_embed_media,
    locale, message_display_compact, native_phone_integration_enabled, render_embeds,
    render_reactions, restricted_guilds, show_current_game, status,
    stream_notifications_enabled, theme, timezone_offset
)
VALUES
    (7251169922837909504, 300, true, true, 1, true, true, 'Working on project', false, true, true, false,
     true, 2, 'friends', true, true, 'folder_data', 'positions_data', true, true,
     'en-US', false, true, true, true, 'restricted_guilds_data', true, 'online', true,
     'dark', -7),
    (7251169922837909505, 600, false, false, 0, false, false, 'Available', false, false, false, true,
     false, 0, NULL, false, false, NULL, NULL, false, false, 'fr-FR', true, false,
     false, false, NULL, false, 'away', false, 'light', 1);

INSERT INTO users (
    id, username, discriminator, avatar, accent_color, banner, theme_colors, pronouns,
    phone, desktop, mobile, premium, premium_type, bot, bio, system, nsfw_allowed,
    mfa_enabled, webauthn_enabled, totp_secret, totp_last_ticket, created_at,
    premium_since, verified, disabled, deleted, email, flags, public_flags,
    purchased_flags, premium_usage_flags, rights, data, fingerprints, extended_settings,
    settings_index
)
VALUES
    (7250861145186111490, 'john_doe_notes', '1296', 'avatar1.png', 16777215, 'banner1.png', NULL,
     'he/him', '+1234567890', true, false, true, 1, false, 'This is my bio', false,
     true, true, true, 'secret123', 'ticket456', '2024-01-01 12:00:00',
     '2024-01-02 12:00:00', true, false, false, 'john_doe_notes@example.com', 100, 10,
     5, 1, 100, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint1', '{"setting": "value"}', 7251169922837909504
     ),
    (7250861145186111491, 'jane_smith_notes', '5678', 'avatar2.png', 123456, NULL, NULL,
     'she/her', NULL, false, true, false, 0, false, '', false,
     false, false, false, NULL, NULL, '2024-02-01 15:30:00',
     NULL, false, false, false, 'jane_smith_notes@example.com', 50, 5,
     0, 0, 50, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint2', '{"setting": "another_value"}', 7251169922837909505
     );
COMMIT;

INSERT INTO notes (
    content, author_id, target_id
)
VALUES 
    ('This is a note', 7250861145186111490, 7250861145186111491),
    ('This is another note', 7250861145186111491, 7250861145186111490);