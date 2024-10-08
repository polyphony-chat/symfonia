create table if not exists message_stickers
(
    messagesId numeric(20, 0) not null constraint chk_messages_id_range check (messagesId >= 0 AND messagesId <= 18446744073709551615),
    stickersId numeric(20, 0) not null constraint chk_stickers_id_range check (stickersId >= 0 AND stickersId <= 18446744073709551615),
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
