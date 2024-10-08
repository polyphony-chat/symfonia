create table if not exists teams
(
    id            numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    icon          varchar(255) null,
    name          varchar(255) not null,
    owner_user_id numeric(20, 0) null constraint chk_owner_user_id_range check (owner_user_id >= 0 AND owner_user_id <= 18446744073709551615),
    constraint FK_13f00abf7cb6096c43ecaf8c108
        foreign key (owner_user_id) references users (id)
);
