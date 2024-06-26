create table if not exists audit_logs
(
    id          varchar(255) not null
        primary key,
    user_id     varchar(255) null,
    guild_id    varchar(255) not null,
    action_type int          not null,
    options     text         null,
    changes     text         not null,
    reason      varchar(255) null,
    target_id   varchar(255) null,
    constraint FK_3cd01cd3ae7aab010310d96ac8e
        foreign key (target_id) references users (id),
    constraint FK_bd2726fd31b35443f2245b93ba0
        foreign key (user_id) references users (id)
);