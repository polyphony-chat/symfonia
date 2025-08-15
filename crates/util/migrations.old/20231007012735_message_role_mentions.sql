create table if not exists message_role_mentions
(
    messagesid numeric(20, 0) not null constraint chk_messages_id_range check (
        messagesid >= 0 and messagesid <= 18446744073709551615
    ),
    rolesid numeric(20, 0) not null constraint chk_roles_id_range check (
        rolesid >= 0 and rolesid <= 18446744073709551615
    ),
    primary key (messagesid, rolesid),
    constraint fk_29d63eb1a458200851bc37d074b
    foreign key (rolesid) references roles (id)
    on update cascade on delete cascade,
    constraint fk_a8242cf535337a490b0feaea0b4
    foreign key (messagesid) references messages (id)
    on update cascade on delete cascade
);

create index if not exists idx_29d63eb1a458200851bc37d074
on message_role_mentions (rolesid);

create index if not exists idx_a8242cf535337a490b0feaea0b
on message_role_mentions (messagesid);
