create table if not exists bans
(
    id          varchar(255) not null
        primary key,
    user_id     varchar(255) null,
    guild_id    varchar(255) null,
    executor_id varchar(255) null,
    ip          varchar(255) not null,
    reason      varchar(255) null,
    constraint FK_07ad88c86d1f290d46748410d58
        foreign key (executor_id) references users (id),
    constraint FK_5999e8e449f80a236ff72023559
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_9d3ab7dd180ebdd245cdb66ecad
        foreign key (guild_id) references guilds (id)
            on delete cascade
);