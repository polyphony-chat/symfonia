create table if not exists rate_limits
(
    id          varchar(255) not null
        primary key,
    executor_id varchar(255) not null,
    hits        int          not null,
    blocked     tinyint      not null,
    expires_at  datetime     not null
);