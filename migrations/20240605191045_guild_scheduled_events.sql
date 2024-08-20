create table guild_scheduled_events
(
    id                   varchar(255)                          not null
        primary key,
    guild_id             varchar(255)                          not null,
    channel_id           varchar(255)                          null,
    creator_id           varchar(255)                          null,
    name                 varchar(100)                          null,
    description          text                                  null,
    scheduled_start_time timestamp default CURRENT_TIMESTAMP   not null,
    scheduled_end_time   timestamp                             null,
    privacy_level        int                                   not null,
    status               int                                   not null,
    entity_type          int                                   not null,
    entity_id            varchar(255)                          null,
    location             varchar(100)                          null,
    user_count           int       default 0                   not null,
    image                text                                  null,
    constraint guild_scheduled_event_channels_id_fk
        foreign key (channel_id) references channels (id),
    constraint guild_scheduled_event_guilds_id_fk
        foreign key (guild_id) references guilds (id),
    constraint guild_scheduled_event_users_id_fk
        foreign key (creator_id) references users (id)
);

create or replace function update_scheduled_start_time()
returns TRIGGER as $$
begin
    NEW.scheduled_start_time := CURRENT_TIMESTAMP;
    return NEW;
end;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_scheduled_start_time_trigger
BEFORE UPDATE ON guild_scheduled_events
FOR EACH ROW
EXECUTE FUNCTION update_scheduled_start_time();