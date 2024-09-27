CREATE SEQUENCE members_index_seq;

create table if not exists members
(
    index                        numeric(20, 0)     not null default nextval('members_index_seq') constraint chk_index_range check (index >= 0 and index <= 18446744073709551615) primary key,
    id                           numeric(20, 0)     not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615),
    guild_id                     numeric(20, 0)     not null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    nick                         varchar(255)       null,
    joined_at                    timestamp          not null,
    premium_since                bigint             null,
    deaf                         boolean            not null,
    mute                         boolean            not null,
    pending                      boolean            not null,
    settings                     text               not null,
    last_message_id              numeric(20, 0)     null constraint chk_last_message_id_range check (last_message_id >= 0 AND last_message_id <= 18446744073709551615),
    joined_by                    varchar(255)       null,
    avatar                       varchar(255)       null,
    banner                       varchar(255)       null,
    bio                          varchar(255)       not null,
    theme_colors                 text               null,
    pronouns                     varchar(255)       null,
    communication_disabled_until timestamp          null,
    constraint IDX_bb2bf9386ac443afbbbf9f12d3
        unique (id, guild_id),
    constraint FK_16aceddd5b89825b8ed6029ad1c
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_28b53062261b996d9c99fa12404
        foreign key (id) references users (id)
            on delete cascade
);

ALTER SEQUENCE members_index_seq OWNED BY members.index;