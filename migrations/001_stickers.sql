create table if not exists stickers
(
    id          numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    name        varchar(255) not null,
    description varchar(255) null,
    available   boolean       null,
    tags        varchar(255) null,
    pack_id     numeric(20, 0) null constraint chk_pack_id_range check (pack_id >= 0 AND pack_id <= 18446744073709551615),
    guild_id    numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    user_id     numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    type        int          not null,
    format_type int          not null,
    constraint FK_193d551d852aca5347ef5c9f205
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_8f4ee73f2bb2325ff980502e158
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_e7cfa5cefa6661b3fb8fda8ce69
        foreign key (pack_id) references sticker_packs (id)
            on delete cascade
);

alter table sticker_packs
    add constraint FK_448fafba4355ee1c837bbc865f1
        foreign key (coverStickerId) references stickers (id);
