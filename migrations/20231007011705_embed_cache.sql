create table if not exists embed_cache
(
    id    varchar(255) not null
        primary key,
    url   varchar(255) not null,
    embed text         not null
);