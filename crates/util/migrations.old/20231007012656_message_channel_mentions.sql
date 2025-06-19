create table if not exists message_channel_mentions
(
    messagesid numeric(20, 0) not null constraint chk_messages_id_range check (
        messagesid >= 0 and messagesid <= 18446744073709551615
    ),
    channelsid numeric(20, 0) not null constraint chk_channels_id_range check (
        channelsid >= 0 and channelsid <= 18446744073709551615
    ),
    primary key (messagesid, channelsid),
    constraint fk_2a27102ecd1d81b4582a4360921
    foreign key (messagesid) references messages (id)
    on update cascade on delete cascade,
    constraint fk_bdb8c09e1464cabf62105bf4b9d
    foreign key (channelsid) references channels (id)
    on update cascade on delete cascade
);

create index if not exists idx_2a27102ecd1d81b4582a436092
on message_channel_mentions (messagesid);

create index if not exists idx_bdb8c09e1464cabf62105bf4b9
on message_channel_mentions (channelsid);
