create table if not exists relationships
(
    id       varchar(255) not null
        primary key,
    from_id  varchar(255) not null,
    to_id    varchar(255) not null,
    nickname varchar(255) null,
    type     int          not null,
    constraint IDX_a0b2ff0a598df0b0d055934a17
        unique (from_id, to_id),
    constraint FK_9af4194bab1250b1c584ae4f1d7
        foreign key (from_id) references users (id)
            on delete cascade,
    constraint FK_9c7f6b98a9843b76dce1b0c878b
        foreign key (to_id) references users (id)
            on delete cascade
);