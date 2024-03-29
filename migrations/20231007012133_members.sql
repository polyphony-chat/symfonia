create table if not exists members
(
    `index`                      int auto_increment
        primary key,
    id                           varchar(255) not null,
    guild_id                     varchar(255) not null,
    nick                         varchar(255) null,
    joined_at                    datetime     not null,
    premium_since                bigint       null,
    deaf                         tinyint      not null,
    mute                         tinyint      not null,
    pending                      tinyint      not null,
    settings                     text         not null,
    last_message_id              varchar(255) null,
    joined_by                    varchar(255) null,
    avatar                       varchar(255) null,
    banner                       varchar(255) null,
    bio                          varchar(255) not null,
    theme_colors                 text         null,
    pronouns                     varchar(255) null,
    communication_disabled_until datetime     null,
    constraint IDX_bb2bf9386ac443afbbbf9f12d3
        unique (id, guild_id),
    constraint FK_16aceddd5b89825b8ed6029ad1c
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_28b53062261b996d9c99fa12404
        foreign key (id) references users (id)
            on delete cascade
);