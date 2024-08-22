create table if not exists stage_instances
(
    id                       int                  not null          primary key,
    guild_id                 varchar(255)         not null,
    channel_id               varchar(255)         not null,
    topic                    varchar(120)         not null,
    privacy_level            int                  not null,
    invite_code              varchar(16)          null,
    discoverable_disabled    smallint             not null default 0,
    guild_scheduled_event_id varchar(255)         null,
    constraint stage_instances_channels_id_fk
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint stage_instances_guilds_id_fk
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint stage_instances_guild_scheduled_events_id_fk
        foreign key (guild_scheduled_event_id) references guild_scheduled_events (id)
);

