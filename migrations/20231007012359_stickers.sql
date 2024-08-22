create table if not exists stickers
(
    id          varchar(255) not null
        primary key,
    name        varchar(255) not null,
    description varchar(255) null,
    available   smallint      null,
    tags        varchar(255) null,
    pack_id     varchar(255) null,
    guild_id    varchar(255) null,
    user_id     varchar(255) null,
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