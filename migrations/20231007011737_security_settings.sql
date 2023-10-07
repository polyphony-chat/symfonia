create table if not exists security_settings
(
    id                         varchar(255) not null
        primary key,
    guild_id                   varchar(255) null,
    channel_id                 varchar(255) null,
    encryption_permission_mask int          not null,
    allowed_algorithms         text         not null,
    current_algorithm          varchar(255) not null,
    used_since_message         varchar(255) null
);