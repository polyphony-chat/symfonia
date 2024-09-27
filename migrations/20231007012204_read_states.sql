create table if not exists read_states
(
    channel_id           numeric(20, 0) not null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    user_id              numeric(20, 0) not null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    last_message_id      numeric(20, 0) null constraint chk_last_message_id_range check (last_message_id >= 0 AND last_message_id <= 18446744073709551615),
    public_ack           varchar(255) null,
    notifications_cursor numeric(20, 0) null constraint chk_notifications_cursor_range check (notifications_cursor >= 0 AND notifications_cursor <= 18446744073709551615),
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
