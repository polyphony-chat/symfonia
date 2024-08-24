create table if not exists roles
(
    id            numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    guild_id      numeric(20, 0) not null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    color         int           not null,
    hoist         boolean       not null,
    managed       boolean       not null,
    mentionable   boolean       not null,
    name          varchar(255)  not null,
    permissions   varchar(255)  not null,
    position      int           not null,
    icon          varchar(255)  null,
    unicode_emoji varchar(255)  null,
    tags          text          null,
    flags         int default 0 not null,
    constraint FK_c32c1ab1c4dc7dcb0278c4b1b8b
        foreign key (guild_id) references guilds (id)
            on delete cascade
);
