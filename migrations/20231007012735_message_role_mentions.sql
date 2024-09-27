create table if not exists message_role_mentions
(
    messagesId numeric(20, 0) not null constraint chk_messages_id_range check (messagesId >= 0 AND messagesId <= 18446744073709551615),
    rolesId    numeric(20, 0) not null constraint chk_roles_id_range check (rolesId >= 0 AND rolesId <= 18446744073709551615),
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
