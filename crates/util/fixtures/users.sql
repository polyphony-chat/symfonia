-- Inserts 5 user_settings records and 5 users records into their respective tables.
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
    (1, 300, true, true, 1, true, true, 'Working on project', false, true, true, false, 
     true, 2, 'friends', true, true, 'folder_data', 'positions_data', true, true, 
     'en-US', false, true, true, true, 'restricted_guilds_data', true, 'online', true, 
     'dark', -7),
    (2, 600, false, false, 0, false, false, 'Available', false, false, false, true, 
     false, 0, NULL, false, false, NULL, NULL, false, false, 'fr-FR', true, false, 
     false, false, NULL, false, 'away', false, 'light', 1),
    (3, 120, false, true, 1, true, true, 'Busy coding', true, true, true, false, 
    true, 2, 'friends_data', true, true, 'folder3_data', 'position3_data', true, true, 
    'es-ES', false, true, true, true, 'restricted_guilds_data3', false, 'dnd', true, 
    'dark', -5),
    (4, 240, true, true, 2, false, false, 'Gaming mode', false, true, false, false, 
     false, 1, 'flags_data', false, true, 'folder4_data', 'position4_data', false, false, 
     'de-DE', true, true, true, false, NULL, true, 'online', false, 'dark', 0),
    (5, 180, false, false, 0, true, true, 'On a break', false, false, true, true, 
     false, 0, 'source_flags', true, false, NULL, NULL, true, true, 
     'ja-JP', false, false, false, true, 'restricted_guilds5', false, 'idle', false, 
     'light', 9);

INSERT INTO users (
    id, username, discriminator, avatar, accent_color, banner, theme_colors, pronouns, 
    phone, desktop, mobile, premium, premium_type, bot, bio, system, nsfw_allowed, 
    mfa_enabled, webauthn_enabled, totp_secret, totp_last_ticket, created_at, 
    premium_since, verified, disabled, deleted, email, flags, public_flags, 
    purchased_flags, premium_usage_flags, rights, data, fingerprints, extended_settings, 
    settings_index
)
VALUES 
    (7248639845155737600, 'john_doe', '1296', 'avatar1.png', 16777215, 'banner1.png', NULL, 
     'he/him', '+1234567890', true, false, true, 1, false, 'This is my bio', false, 
     true, true, true, 'secret123', 'ticket456', '2024-01-01 12:00:00', 
     '2024-01-02 12:00:00', true, false, false, 'john_doe@example.com', 100, 10, 
     5, 1, 100, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint1', '{"setting": "value"}', 1
     ),
    (7248639891561517057, 'jane_smith', '5678', 'avatar2.png', 123456, NULL, NULL, 
     'she/her', NULL, false, true, false, 0, false, '', false, 
     false, false, false, NULL, NULL, '2024-02-01 15:30:00', 
     NULL, false, false, false, 'jane_smith@example.com', 50, 5, 
     0, 0, 50, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint2', '{"setting": "another_value"}', 2
     ),
    (7248640296244744192, 'alice_malice', '9876', 'avatar3.png', 987654, 'banner3.png', NULL, 
    'they/them', '+1230987654', true, true, false, 0, false, 'Life is an adventure', 
    false, false, true, false, 'secret789', 'ticket123', '2024-03-01 08:45:00', 
    NULL, false, false, false, 'alice@example.com', 200, 20, 
    2, 1, 150, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint3', '{"setting3": "value3"}', 3
    ),
    (7248640311927246848, 'bob_the_woz', '1122', 'avatar4.png', 556677, NULL, NULL, 
     'he/him', NULL, true, false, true, 1, false, 'Building the future', 
     false, true, false, true, 'secret456', 'ticket789', '2024-03-15 14:30:00', 
     '2024-04-01 14:30:00', true, false, false, 'bob@example.com', 250, 25, 
     3, 2, 200, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint4', '{"setting4": "value4"}', 4
     ),
    (7248640327265816576, 'xenia', '3344', 'avatar5.png', NULL, 'banner5.png', NULL, 
     'he/him', '+4567890123', false, true, false, 0, false, 'Chocolate is life', 
     false, false, false, true, 'secret555', 'ticket987', '2024-04-10 10:00:00', 
     NULL, false, false, false, 'charlie@example.com', 180, 15, 
     0, 0, 180, '{"hash": "$2b$14$AbcNYTh5GcOYaB4v4P7OWeGW0hQkd/ysvmY4fGyIA/tyEnK0chGNu", "valid_tokens_since": "2024-10-07T20:22:06.162070616Z"}', 'fingerprint5', '{"setting5": "value5"}', 5
     );
COMMIT;