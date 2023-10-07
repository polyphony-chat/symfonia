create table if not exists attachments
(
    id           varchar(255) not null
        primary key,
    filename     varchar(255) not null,
    size         int          not null,
    url          varchar(255) not null,
    proxy_url    varchar(255) not null,
    height       int          null,
    width        int          null,
    content_type varchar(255) null,
    message_id   varchar(255) null,
    constraint FK_623e10eec51ada466c5038979e3
        foreign key (message_id) references messages (id)
            on delete cascade
);