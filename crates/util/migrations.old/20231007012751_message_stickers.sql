create table if not exists message_stickers
(
    messagesid numeric(20, 0) not null constraint chk_messages_id_range check (
        messagesid >= 0 and messagesid <= 18446744073709551615
    ),
    stickersid numeric(20, 0) not null constraint chk_stickers_id_range check (
        stickersid >= 0 and stickersid <= 18446744073709551615
    ),
    primary key (messagesid, stickersid),
    constraint fk_40bb6f23e7cc133292e92829d28
    foreign key (messagesid) references messages (id)
    on update cascade on delete cascade,
    constraint fk_e22a70819d07659c7a71c112a1f
    foreign key (stickersid) references stickers (id)
    on update cascade on delete cascade
);

create index if not exists idx_40bb6f23e7cc133292e92829d2
on message_stickers (messagesid);

create index if not exists idx_e22a70819d07659c7a71c112a1
on message_stickers (stickersid);
