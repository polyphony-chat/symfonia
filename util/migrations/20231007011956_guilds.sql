    create table if not exists guilds
    (
        id                            numeric(20, 0)    not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
        afk_channel_id                numeric(20, 0)    null constraint chk_afk_channel_range check (afk_channel_id >= 0 AND afk_channel_id <= 18446744073709551615),
        afk_timeout                   int               null,
        banner                        varchar(255)      null,
        default_message_notifications int               null,
        description                   varchar(255)      null,
        discovery_splash              varchar(255)      null,
        explicit_content_filter       int               null,
        features                      jsonb             not null,
        primary_category_id           varchar(255)      null,
        icon                          varchar(255)      null,
        large                         boolean           not null,
        max_members                   int               null,
        max_presences                 int               null,
        max_video_channel_users       int               null,
        member_count                  int               null,
        presence_count                int               null,
        template_id                   numeric(20, 0)    null constraint chk_template_range check (template_id >= 0 AND template_id <= 18446744073709551615),
        mfa_level                     int               null,
        name                          varchar(255)      not null,
        owner_id                      numeric(20, 0)    null constraint chk_owner_id_range check (owner_id >= 0 AND owner_id <= 18446744073709551615),
        preferred_locale              varchar(255)      null,
        premium_subscription_count    int               null,
        premium_tier                  int               not null,
        public_updates_channel_id     numeric(20, 0)    null constraint check_pub_upd_channel_id check (public_updates_channel_id >= 0 AND public_updates_channel_id <= 18446744073709551615),
        rules_channel_id              numeric(20, 0)    null constraint chk_rules_channel_id check (rules_channel_id >= 0 AND rules_channel_id <= 18446744073709551615),
        region                        varchar(255)      null,
        splash                        varchar(255)      null,
        system_channel_id             numeric(20, 0)    null constraint chk_system_channel_id check (system_channel_id >= 0 AND system_channel_id <= 18446744073709551615),
        system_channel_flags          int               null,
        unavailable                   boolean           not null,
        verification_level            int               null,
        welcome_screen                jsonb             not null,
        widget_channel_id             numeric(20, 0)    null constraint chk_widget_channel_id check (widget_channel_id >= 0 AND widget_channel_id <= 18446744073709551615),
        widget_enabled                boolean           not null,
        nsfw_level                    int               null,
        nsfw                          boolean           not null,
        parent                        numeric(20, 0)    null constraint chk_parent_id check (parent >= 0 AND parent <= 18446744073709551615),
        premium_progress_bar_enabled  boolean           null,
        constraint FK_public_updates_channel_id
            foreign key (public_updates_channel_id) references channels (id),
        constraint FK_rules_channel_id
            foreign key (rules_channel_id) references channels (id),
        constraint FK_widget_channel_id
            foreign key (widget_channel_id) references channels (id),
        constraint FK_system_channel_id
            foreign key (system_channel_id) references channels (id),
        constraint FK_afk_channel_id
            foreign key (afk_channel_id) references channels (id),
        constraint FK_owner_user_id
            foreign key (owner_id) references users (id)
    );