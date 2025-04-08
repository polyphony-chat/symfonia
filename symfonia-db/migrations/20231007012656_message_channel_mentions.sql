create table if not exists message_channel_mentions
(
    messagesId numeric(20, 0) not null constraint chk_messages_id_range check (messagesId >= 0 AND messagesId <= 18446744073709551615),
    channelsId numeric(20, 0) not null constraint chk_channels_id_range check (channelsId >= 0 AND channelsId <= 18446744073709551615),
    primary key (messagesId, channelsId),
    constraint FK_2a27102ecd1d81b4582a4360921
        foreign key (messagesId) references messages (id)
            on update cascade on delete cascade,
    constraint FK_bdb8c09e1464cabf62105bf4b9d
        foreign key (channelsId) references channels (id)
            on update cascade on delete cascade
);

create index if not exists IDX_2a27102ecd1d81b4582a436092
    on message_channel_mentions (messagesId);

create index if not exists IDX_bdb8c09e1464cabf62105bf4b9
    on message_channel_mentions (channelsId);
