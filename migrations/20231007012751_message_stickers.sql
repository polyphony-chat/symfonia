create table if not exists message_stickers
(
    messagesId varchar(255) not null,
    stickersId varchar(255) not null,
    primary key (messagesId, stickersId),
    constraint FK_40bb6f23e7cc133292e92829d28
        foreign key (messagesId) references messages (id)
            on update cascade on delete cascade,
    constraint FK_e22a70819d07659c7a71c112a1f
        foreign key (stickersId) references stickers (id)
            on update cascade on delete cascade
);

create index if not exists IDX_40bb6f23e7cc133292e92829d2
    on message_stickers (messagesId);

create index if not exists IDX_e22a70819d07659c7a71c112a1
    on message_stickers (stickersId);