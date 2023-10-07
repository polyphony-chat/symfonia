create table if not exists security_keys
(
    id         varchar(255) not null
        primary key,
    user_id    varchar(255) null,
    key_id     varchar(255) not null,
    public_key varchar(255) not null,
    counter    int          not null,
    name       varchar(255) not null,
    constraint FK_24c97d0771cafedce6d7163eaad
        foreign key (user_id) references users (id)
            on delete cascade
);