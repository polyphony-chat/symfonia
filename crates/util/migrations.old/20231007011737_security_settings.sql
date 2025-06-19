create table if not exists security_settings
(
    id                         numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    guild_id                   numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    channel_id                 numeric(20, 0) null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    encryption_permission_mask int          not null,
    allowed_algorithms         text         not null,
    current_algorithm          varchar(255) not null,
    used_since_message         varchar(255) null
);