create table if not exists connected_accounts
(
    id                  varchar(255) not null
        primary key,
    external_id         varchar(255) not null,
    user_id             varchar(255) null,
    friend_sync         tinyint      not null,
    name                varchar(255) not null,
    revoked             tinyint      not null,
    show_activity       int          not null,
    type                varchar(255) not null,
    verified            tinyint      not null,
    visibility          int          not null,
    integrations        text         not null,
    metadata            text         null,
    metadata_visibility int          not null,
    two_way_link        tinyint      not null,
    token_data          text         null,
    constraint FK_f47244225a6a1eac04a3463dd90
        foreign key (user_id) references users (id)
            on delete cascade
);