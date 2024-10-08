create table if not exists recipients
(
    id         numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    channel_id numeric(20, 0) not null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    user_id    numeric(20, 0) not null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    closed     boolean default false not null,
    constraint FK_2f18ee1ba667f233ae86c0ea60e
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint FK_6157e8b6ba4e6e3089616481fe2
        foreign key (user_id) references users (id)
            on delete cascade
);
