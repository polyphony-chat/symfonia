create table if not exists guild_scheduled_events
(
    id                   numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    guild_id             numeric(20, 0) not null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    channel_id           numeric(20, 0) null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    creator_id           numeric(20, 0) null constraint chk_creator_id_range check (creator_id >= 0 AND creator_id <= 18446744073709551615),
    name                 varchar(100) null,
    description          text null,
    scheduled_start_time timestamp default CURRENT_TIMESTAMP not null,
    scheduled_end_time   timestamp null,
    privacy_level        int not null,
    status               int not null,
    entity_type          int not null,
    entity_id            numeric(20, 0) null constraint chk_entity_id_range check (entity_id >= 0 AND entity_id <= 18446744073709551615),
    location             varchar(100) null,
    user_count           int default 0 not null,
    image                text null,
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
