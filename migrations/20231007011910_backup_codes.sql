create table if not exists backup_codes
(
    id       numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    code     varchar(255) not null,
    consumed boolean       not null,
    expired  boolean       not null,
    user_id  numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    constraint FK_70066ea80d2f4b871beda32633b
        foreign key (user_id) references users (id)
            on delete cascade
);