create table if not exists teams
(
    id            varchar(255) not null
        primary key,
    icon          varchar(255) null,
    name          varchar(255) not null,
    owner_user_id varchar(255) null,
    constraint FK_13f00abf7cb6096c43ecaf8c108
        foreign key (owner_user_id) references users (id)
);