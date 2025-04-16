create table if not exists emojis
(
    id              numeric(20, 0)  not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    animated        boolean       not null,
    available       boolean       not null,
    guild_id        numeric(20, 0) not null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    user_id         numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    managed         boolean       not null,
    name            varchar(255) not null,
    require_colons  boolean       not null,
    roles           text         not null,
    groups          text         null,
    constraint FK_4b988e0db89d94cebcf07f598cc
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_fa7ddd5f9a214e28ce596548421
        foreign key (user_id) references users (id)
);

ALTER TABLE channels
ADD CONSTRAINT FK_emoji_id_emojis_id FOREIGN KEY (default_reaction_emoji) REFERENCES emojis(id);
