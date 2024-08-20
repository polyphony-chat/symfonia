create table if not exists roles
(
    id            varchar(255)  not null
        primary key,
    guild_id      varchar(255)  not null,
    color         int           not null,
    hoist         smallint       not null,
    managed       smallint       not null,
    mentionable   smallint       not null,
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