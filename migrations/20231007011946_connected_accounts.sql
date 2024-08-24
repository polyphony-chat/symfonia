create table if not exists connected_accounts
(
    id                  varchar(255) not null
        primary key,
    external_id         varchar(255) not null,
    user_id             numeric(20, 0)  null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    friend_sync         boolean       not null,
    name                varchar(255) not null,
    revoked             boolean       not null,
    show_activity       int          not null,
    type                varchar(255) not null,
    verified            boolean       not null,
    visibility          int          not null,
    integrations        text         not null,
    metadata            text         null,
    metadata_visibility int          not null,
    two_way_link        boolean       not null,
    token_data          text         null,
    constraint FK_f47244225a6a1eac04a3463dd90
        foreign key (user_id) references users (id)
            on delete cascade
);