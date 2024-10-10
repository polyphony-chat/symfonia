INSERT INTO guilds (
    id, afk_channel_id, afk_timeout, banner, default_message_notifications, 
    description, discovery_splash, explicit_content_filter, features, 
    primary_category_id, icon, large, max_members, max_presences, 
    max_video_channel_users, member_count, presence_count, template_id, 
    mfa_level, name, owner_id, preferred_locale, premium_subscription_count, 
    premium_tier, public_updates_channel_id, rules_channel_id, region, 
    splash, system_channel_id, system_channel_flags, unavailable, 
    verification_level, welcome_screen, widget_channel_id, widget_enabled, 
    nsfw_level, nsfw, parent, premium_progress_bar_enabled
)  
VALUES (
    7249086638293258240, NULL, 300, 'banner-image.png', 1,
    'The first guild', 'splash-image.png', 2, '["COMMUNITY", "NEWS"]',
    'community', 'icon-image.png', TRUE, 5000, 500, 
    50, NULL, NULL, NULL, 1, 'A Guild', NULL,
    'en-US', 0, 2, NULL, NULL, 'us-west',
    'splash-image.png', NULL, 0, FALSE,
    2, '{}', NULL, TRUE,
    1, TRUE, NULL, TRUE
);

INSERT INTO channels (
	id, created_at, name, icon, type, last_message_id, guild_id, parent_id, owner_id, last_pin_timestamp,
    default_auto_archive_duration, "position", permission_overwrites, video_quality_mode, bitrate, user_limit, nsfw,
    rate_limit_per_user, topic, retention_policy_id, flags, default_thread_rate_limit_per_user
)
VALUES (
    7249086862017433600, current_timestamp, 'general', NULL, 0, NULL, 7249086638293258240, NULL,
    NULL, NULL, NULL, 0, NULL,NULL, NULL, NULL, false, 0, 'A channel in a guild. Exciting!', NULL,
    0, 0
);

INSERT INTO guilds (
    id, afk_channel_id, afk_timeout, banner, default_message_notifications, 
    description, discovery_splash, explicit_content_filter, features, 
    primary_category_id, icon, large, max_members, max_presences, 
    max_video_channel_users, member_count, presence_count, template_id, 
    mfa_level, name, owner_id, preferred_locale, premium_subscription_count, 
    premium_tier, public_updates_channel_id, rules_channel_id, region, 
    splash, system_channel_id, system_channel_flags, unavailable, 
    verification_level, welcome_screen, widget_channel_id, widget_enabled, 
    nsfw_level, nsfw, parent, premium_progress_bar_enabled
)  
VALUES (
    7249112309484752896, NULL, 300, 'banner-image.png', 1,
    'The second guild', 'splash-image.png', 2, '["COMMUNITY", "NEWS"]',
    'community', 'icon-image.png', TRUE, 5000, 500, 
    50, NULL, NULL, NULL, 1, 'Another Guild', NULL,
    'de-DE', 0, 2, NULL, NULL, 'us-west',
    'splash-image.png', NULL, 0, FALSE,
    2, '{}', NULL, TRUE,
    1, TRUE, NULL, TRUE
);

INSERT INTO channels (
	id, created_at, name, icon, type, last_message_id, guild_id, parent_id, owner_id, last_pin_timestamp,
    default_auto_archive_duration, "position", permission_overwrites, video_quality_mode, bitrate, user_limit, nsfw,
    rate_limit_per_user, topic, retention_policy_id, flags, default_thread_rate_limit_per_user
)
VALUES (
    7249112493841190912, current_timestamp, 'general', NULL, 0, NULL, 7249112309484752896, NULL,
    NULL, NULL, NULL, 0, NULL,NULL, NULL, NULL, false, 0, 'A channel in the second guild. Exciting!', NULL,
    0, 0
);