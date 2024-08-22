create table if not exists backup_codes
(
    id       varchar(255) not null
        primary key,
    code     varchar(255) not null,
    consumed smallint      not null,
    expired  smallint      not null,
    user_id  varchar(255) null,
    constraint FK_70066ea80d2f4b871beda32633b
        foreign key (user_id) references users (id)
            on delete cascade
);