create table if not exists webhooks
(
    id              varchar(255) not null
        primary key,
    type            int          not null,
    name            varchar(255) null,
    avatar          varchar(255) null,
    token           varchar(255) null,
    guild_id        varchar(255) null,
    channel_id      varchar(255) null,
    application_id  varchar(255) null,
    user_id         varchar(255) null,
    source_guild_id varchar(255) null,
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