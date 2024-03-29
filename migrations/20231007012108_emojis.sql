create table if not exists emojis
(
    id             varchar(255) not null
        primary key,
    animated       tinyint      not null,
    available      tinyint      not null,
    guild_id       varchar(255) not null,
    user_id        varchar(255) null,
    managed        tinyint      not null,
    name           varchar(255) not null,
    require_colons tinyint      not null,
    roles          text         not null,
    `groups`       text         null,
    constraint FK_4b988e0db89d94cebcf07f598cc
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_fa7ddd5f9a214e28ce596548421
        foreign key (user_id) references users (id)
);