create table if not exists attachments
(
    id           numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    filename     varchar(255) not null,
    size         int not null,
    url          varchar(255) not null,
    proxy_url    varchar(255) not null,
    height       int null,
    width        int null,
    content_type varchar(255) null,
    message_id   numeric(20, 0) null constraint chk_message_id_range check (message_id >= 0 AND message_id <= 18446744073709551615),
    constraint FK_623e10eec51ada466c5038979e3
        foreign key (message_id) references messages (id)
            on delete cascade
);
