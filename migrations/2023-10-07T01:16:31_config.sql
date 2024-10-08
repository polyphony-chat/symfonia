create table if not exists config
(
    key varchar(255) not null
        primary key,
    value json         null
);