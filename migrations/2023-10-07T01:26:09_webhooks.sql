create table if not exists webhooks
(
    id              numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    type            int          not null,
    name            varchar(255) null,
    avatar          varchar(255) null,
    token           varchar(255) null,
    guild_id        numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    channel_id      numeric(20, 0) null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    application_id  numeric(20, 0) null constraint chk_application_id_range check (application_id >= 0 AND application_id <= 18446744073709551615),
    user_id         numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    source_guild_id numeric(20, 0) null constraint chk_source_guild_id_range check (source_guild_id >= 0 AND source_guild_id <= 18446744073709551615),
    constraint FK_0d523f6f997c86e052c49b1455f
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_3a285f4f49c40e0706d3018bc9f
        foreign key (source_guild_id) references guilds (id)
            on delete cascade,
    constraint FK_487a7af59d189f744fe394368fc
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_c3e5305461931763b56aa905f1c
        foreign key (application_id) references applications (id)
            on delete cascade,
    constraint FK_df528cf77e82f8032230e7e37d8
        foreign key (channel_id) references channels (id)
            on delete cascade
);
