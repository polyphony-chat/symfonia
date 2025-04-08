create table if not exists invites
(
    code             varchar(255) not null primary key,
    type             boolean      not null,
    temporary        boolean      not null,
    uses             int          not null,
    max_uses         int          not null,
    max_age          int          not null,
    created_at       timestamp     not null,
    expires_at       timestamp     null,
    guild_id         numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    channel_id       numeric(20, 0) null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    inviter_id       numeric(20, 0) null constraint chk_inviter_id check (inviter_id >= 0 AND inviter_id <= 18446744073709551615),
    target_user_id   numeric(20, 0) null constraint chk_target_user_id check (target_user_id >= 0 AND target_user_id <= 18446744073709551615),
    target_user_type int          null,
    vanity_url       boolean       null,
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