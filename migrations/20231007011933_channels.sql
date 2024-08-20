create table if not exists channels
(
    id                                 varchar(255) not null
        primary key,
    created_at                         datetime     not null,
    name                               varchar(255) null,
    icon                               text         null,
    type                               int          not null,
    last_message_id                    varchar(255) null,
    guild_id                           varchar(255) null,
    parent_id                          varchar(255) null,
    owner_id                           varchar(255) null,
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