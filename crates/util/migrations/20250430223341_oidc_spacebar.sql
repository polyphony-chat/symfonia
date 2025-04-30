create table if not exists oidc_spacebar
(
    oidc_sub    varchar(255)    primary key not null,
    user_id     numeric(20,0)   unique not null constraint chk_uid check (user_id >= 0 AND user_id <= 18446744073709551615),
    constraint uid_fk
        foreign key (user_id) references users (id)
);
