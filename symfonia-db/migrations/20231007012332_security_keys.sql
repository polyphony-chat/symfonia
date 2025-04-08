create table if not exists security_keys
(
    id         numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    user_id    numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    key_id     numeric(20, 0) not null constraint chk_key_id_range check (key_id >= 0 AND key_id <= 18446744073709551615),
    public_key varchar(255) not null,
    counter    int          not null,
    name       varchar(255) not null,
    constraint FK_24c97d0771cafedce6d7163eaad
        foreign key (user_id) references users (id)
            on delete cascade
);
