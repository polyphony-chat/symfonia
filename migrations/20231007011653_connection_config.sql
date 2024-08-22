create table if not exists connection_config
(
    key varchar(255) not null
        primary key,
    value text         null
);