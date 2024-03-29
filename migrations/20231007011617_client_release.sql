create table if not exists client_release
(
    id       varchar(255) not null
        primary key,
    name     varchar(255) not null,
    pub_date datetime     not null,
    url      varchar(255) not null,
    platform varchar(255) not null,
    enabled  tinyint      not null,
    notes    varchar(255) null
);