create table if not exists voice_states
(
    id                         varchar(255) not null
        primary key,
    guild_id                   varchar(255) null,
    channel_id                 varchar(255) null,
    user_id                    varchar(255) null,
    session_id                 varchar(255) not null,
    token                      varchar(255) null,
    deaf                       tinyint      not null,
    mute                       tinyint      not null,
    self_deaf                  tinyint      not null,
    self_mute                  tinyint      not null,
    self_stream                tinyint      null,
    self_video                 tinyint      not null,
    suppress                   tinyint      not null,
    request_to_speak_timestamp datetime     null,
    constraint FK_03779ef216d4b0358470d9cb748
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_5fe1d5f931a67e85039c640001b
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_9f8d389866b40b6657edd026dd4
        foreign key (channel_id) references channels (id)
            on delete cascade
);