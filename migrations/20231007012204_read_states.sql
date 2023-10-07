create table if not exists read_states
(
    id                   varchar(255) not null
        primary key,
    channel_id           varchar(255) not null,
    user_id              varchar(255) not null,
    last_message_id      varchar(255) null,
    public_ack           varchar(255) null,
    notifications_cursor varchar(255) null,
    last_pin_timestamp   datetime     null,
    mention_count        int          null,
    constraint IDX_0abf8b443321bd3cf7f81ee17a
        unique (channel_id, user_id),
    constraint FK_195f92e4dd1254a4e348c043763
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_40da2fca4e0eaf7a23b5bfc5d34
        foreign key (channel_id) references channels (id)
            on delete cascade
);