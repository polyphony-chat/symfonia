create table if not exists channels
(
    id                                 numeric(20, 0)  not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    created_at                         timestamp     not null,
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
    permission_overwrites              text         null,
    video_quality_mode                 int          null,
    bitrate                            int          null,
    user_limit                         int          null,
    nsfw                               smallint      not null,
    rate_limit_per_user                int          null,
    topic                              varchar(255) null,
    retention_policy_id                varchar(255) null,
    flags                              int          not null,
    default_thread_rate_limit_per_user int          not null,
    constraint FK_3274522d14af40540b1a883fc80
        foreign key (parent_id) references channels (id),
    constraint FK_3873ed438575cce703ecff4fc7b
        foreign key (owner_id) references users (id)
);