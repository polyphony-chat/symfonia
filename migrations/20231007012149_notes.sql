create table if not exists notes
(
    id        varchar(255) not null primary key,
    content   varchar(255) not null,
    owner_id  numeric(20, 0) null constraint chk_owner_id_range check (owner_id >= 0 AND owner_id <= 18446744073709551615),
    target_id numeric(20, 0) null constraint chk_target_id_range check (target_id >= 0 AND target_id <= 18446744073709551615),
    constraint IDX_74e6689b9568cc965b8bfc9150
        unique (owner_id, target_id),
    constraint FK_23e08e5b4481711d573e1abecdc
        foreign key (target_id) references users (id)
            on delete cascade,
    constraint FK_f9e103f8ae67cb1787063597925
        foreign key (owner_id) references users (id)
            on delete cascade
);