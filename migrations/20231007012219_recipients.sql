create table if not exists recipients
(
    id         varchar(255)      not null
        primary key,
    channel_id varchar(255)      not null,
    user_id    varchar(255)      not null,
    closed     smallint default 0 not null,
    constraint FK_2f18ee1ba667f233ae86c0ea60e
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint FK_6157e8b6ba4e6e3089616481fe2
        foreign key (user_id) references users (id)
            on delete cascade
);