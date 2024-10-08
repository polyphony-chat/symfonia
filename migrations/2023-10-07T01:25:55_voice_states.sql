create table if not exists voice_states
(
    id                         numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    guild_id                   numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    channel_id                 numeric(20, 0) null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    user_id                    numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    session_id                 numeric(20, 0) not null constraint chk_session_id_range check (session_id >= 0 AND session_id <= 18446744073709551615),
    token                      varchar(255) null,
    deaf                       boolean       not null,
    mute                       boolean       not null,
    self_deaf                  boolean       not null,
    self_mute                  boolean       not null,
    self_stream                boolean       null,
    self_video                 boolean       not null,
    suppress                   boolean       not null,
    request_to_speak_timestamp timestamp     null,
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
