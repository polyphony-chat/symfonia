create table if not exists rate_limits
(
    id          varchar(255) not null
        primary key,
    executor_id varchar(255) not null,
    hits        int          not null,
    blocked     smallint      not null,
    expires_at  timestamp     not null
);