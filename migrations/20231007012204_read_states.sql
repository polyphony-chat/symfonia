create table if not exists read_states
(
    channel_id           varchar(255) not null,
    user_id              varchar(255) not null,
    last_message_id      varchar(255) null,
    public_ack           varchar(255) null,
    notifications_cursor varchar(255) null,
    last_pin_timestamp   timestamp     null,
    mention_count        int          null,
    constraint read_states_channel_id_user_id_uindex
        unique (channel_id, user_id),
    constraint read_states_users_id_fk
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint read_states_channels_id_fk
        foreign key (channel_id) references channels (id)
            on delete cascade
);