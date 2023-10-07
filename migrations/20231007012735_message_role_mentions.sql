create table if not exists message_role_mentions
(
    messagesId varchar(255) not null,
    rolesId    varchar(255) not null,
    primary key (messagesId, rolesId),
    constraint FK_29d63eb1a458200851bc37d074b
        foreign key (rolesId) references roles (id)
            on update cascade on delete cascade,
    constraint FK_a8242cf535337a490b0feaea0b4
        foreign key (messagesId) references messages (id)
            on update cascade on delete cascade
);

create index if not exists IDX_29d63eb1a458200851bc37d074
    on message_role_mentions (rolesId);

create index if not exists IDX_a8242cf535337a490b0feaea0b
    on message_role_mentions (messagesId);