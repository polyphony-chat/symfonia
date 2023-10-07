create table if not exists valid_registration_tokens
(
    token      varchar(255) not null
        primary key,
    created_at datetime     not null,
    expires_at datetime     not null
);