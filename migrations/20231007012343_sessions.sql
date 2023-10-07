create table if not exists sessions
(
    id          varchar(255) not null
        primary key,
    user_id     varchar(255) null,
    session_id  varchar(255) not null,
    activities  text         null,
    client_info text         not null,
    status      varchar(255) not null,
    constraint FK_085d540d9f418cfbdc7bd55bb19
        foreign key (user_id) references users (id)
            on delete cascade
);