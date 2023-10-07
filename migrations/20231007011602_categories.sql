create table if not exists categories
(
    id            int          not null
        primary key,
    name          varchar(255) null,
    localizations text         not null,
    is_primary    tinyint      null
);