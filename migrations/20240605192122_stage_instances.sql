create table if not exists stage_instances
(
    id                       numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    guild_id                 numeric(20, 0) not null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    channel_id               numeric(20, 0) not null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    topic                    varchar(120) not null,
    privacy_level            int not null,
    invite_code              varchar(16) null,
    discoverable_disabled    boolean  not null default false,
    guild_scheduled_event_id numeric(20, 0) null constraint chk_guild_scheduled_event_id_range check (guild_scheduled_event_id >= 0 AND guild_scheduled_event_id <= 18446744073709551615),
    constraint stage_instances_channels_id_fk
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint stage_instances_guilds_id_fk
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint stage_instances_guild_scheduled_events_id_fk
        foreign key (guild_scheduled_event_id) references guild_scheduled_events (id)
);
