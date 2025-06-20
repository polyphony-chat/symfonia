create table if not exists message_user_mentions
(
    messagesid numeric(20, 0) not null constraint chk_messages_id_range check (
        messagesid >= 0 and messagesid <= 18446744073709551615
    ),
    usersid numeric(20, 0) not null constraint chk_users_id_range check (
        usersid >= 0 and usersid <= 18446744073709551615
    ),
    primary key (messagesid, usersid),
    constraint fk_a343387fc560ef378760681c236
    foreign key (messagesid) references messages (id)
    on update cascade on delete cascade,
    constraint fk_b831eb18ceebd28976239b1e2f8
    foreign key (usersid) references users (id)
    on update cascade on delete cascade
);

create index if not exists idx_a343387fc560ef378760681c23
on message_user_mentions (messagesid);

create index if not exists idx_b831eb18ceebd28976239b1e2f
on message_user_mentions (usersid);

create index if not exists idx_05535bc695e9f7ee104616459d
on messages (author_id);

create index if not exists idx_86b9109b155eb70c0a2ca3b4b6
on messages (channel_id);
