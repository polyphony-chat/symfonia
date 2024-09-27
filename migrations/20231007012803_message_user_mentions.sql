create table if not exists message_user_mentions
(
    messagesId numeric(20, 0) not null constraint chk_messages_id_range check (messagesId >= 0 AND messagesId <= 18446744073709551615),
    usersId    numeric(20, 0) not null constraint chk_users_id_range check (usersId >= 0 AND usersId <= 18446744073709551615),
    primary key (messagesId, usersId),
    constraint FK_a343387fc560ef378760681c236
        foreign key (messagesId) references messages (id)
            on update cascade on delete cascade,
    constraint FK_b831eb18ceebd28976239b1e2f8
        foreign key (usersId) references users (id)
            on update cascade on delete cascade
);

create index if not exists IDX_a343387fc560ef378760681c23
    on message_user_mentions (messagesId);

create index if not exists IDX_b831eb18ceebd28976239b1e2f
    on message_user_mentions (usersId);

create index if not exists IDX_05535bc695e9f7ee104616459d
    on messages (author_id);

create index if not exists IDX_86b9109b155eb70c0a2ca3b4b6
    on messages (channel_id);
