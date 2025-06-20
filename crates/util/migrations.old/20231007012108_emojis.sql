create table if not exists emojis
(
    id numeric(20, 0) not null constraint chk_id_range check (
        id >= 0 and id <= 18446744073709551615
    ) primary key,
    animated boolean not null,
    available boolean not null,
    guild_id numeric(20, 0) not null constraint chk_guild_id_range check (
        guild_id >= 0 and guild_id <= 18446744073709551615
    ),
    user_id numeric(20, 0) null constraint chk_user_id_range check (
        user_id >= 0 and user_id <= 18446744073709551615
    ),
    managed boolean not null,
    name varchar(255) not null,
    require_colons boolean not null,
    roles text not null,
    groups text null,
    constraint fk_4b988e0db89d94cebcf07f598cc
    foreign key (guild_id) references guilds (id)
    on delete cascade,
    constraint fk_fa7ddd5f9a214e28ce596548421
    foreign key (user_id) references users (id)
);

alter table channels
add constraint fk_emoji_id_emojis_id foreign key (
    default_reaction_emoji
) references emojis (id);
