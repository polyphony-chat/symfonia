create table if not exists invites
(
    code             varchar(255) not null
        primary key,
    type             smallint     not null,
    temporary        smallint      not null,
    uses             int          not null,
    max_uses         int          not null,
    max_age          int          not null,
    created_at       datetime     not null,
    expires_at       datetime     null,
    guild_id         varchar(255) null,
    channel_id       varchar(255) null,
    inviter_id       varchar(255) null,
    target_user_id   varchar(255) null,
    target_user_type int          null,
    vanity_url       smallint      null,
    flags            int          not null,
    constraint FK_11a0d394f8fc649c19ce5f16b59
        foreign key (target_user_id) references users (id)
            on delete cascade,
    constraint FK_15c35422032e0b22b4ada95f48f
        foreign key (inviter_id) references users (id)
            on delete cascade,
    constraint FK_3f4939aa1461e8af57fea3fb05d
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_6a15b051fe5050aa00a4b9ff0f6
        foreign key (channel_id) references channels (id)
            on delete cascade
);