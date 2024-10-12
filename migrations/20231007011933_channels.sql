create table if not exists channels
(
    id                                 numeric(20, 0)  not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    created_at                         timestamptz  not null,
    name                               varchar(255) null,
    icon                               text         null,
    type                               int          not null,
    last_message_id                    numeric(20, 0)  null constraint chk_last_message_id_range check (last_message_id >= 0 AND last_message_id <= 18446744073709551615),
    guild_id                           numeric(20, 0)  null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    parent_id                          numeric(20, 0)  null constraint chk_parent_id check (parent_id >= 0 AND parent_id <= 18446744073709551615),
    owner_id                           numeric(20, 0)  null constraint chk_owner_id_range check (owner_id >= 0 AND owner_id <= 18446744073709551615),
    last_pin_timestamp                 int          null,
    default_auto_archive_duration      int          null,
    position                           int          null,
    managed                            boolean      null,
    rtc_region                         varchar(255) null,
    permission_overwrites              text         null,
    video_quality_mode                 int          null,
    bitrate                            int          null,
    user_limit                         int          null,
    nsfw                               boolean       not null,
    rate_limit_per_user                int          null,
    topic                              varchar(255) null,
    retention_policy_id                varchar(255) null,
    flags                              int          not null,
    default_thread_rate_limit_per_user int          not null,
    default_sort_order                 numeric(3, 0) null constraint chk_default_sort_order check (default_sort_order >= 0 AND default_sort_order <= 255),
    -- v foreign key constraint is added in emojis migration
    -- TODO: I am unsure if deserializing a channel object from the database will work with the default_reaction_emoji field. Test it
    default_reaction_emoji             numeric(20, 0) null constraint chk_default_reaction_emoji check (default_reaction_emoji >= 0 AND default_reaction_emoji <= 18446744073709551615),
    default_forum_layout               numeric(3, 0) null constraint chk_default_forum_layout check (default_forum_layout >= 0 AND default_forum_layout <= 255),
    available_tags                     jsonb        null constraint chk_available_tags check (jsonb_typeof(available_tags) = 'array') default '[]',
    applied_tags                       jsonb        null constraint chk_applied_tags check (jsonb_typeof(applied_tags) = 'array') default '[]',
    application_id                     numeric(20, 0)  null constraint chk_application_id_range check (application_id >= 0 AND application_id <= 18446744073709551615),
    constraint FK_3274522d14af40540b1a883fc80
        foreign key (parent_id) references channels (id),
    constraint FK_3873ed438575cce703ecff4fc7b
        foreign key (owner_id) references users (id)
);