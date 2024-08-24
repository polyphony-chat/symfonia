create table if not exists relationships
(
    id       numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    from_id  numeric(20, 0) not null constraint chk_from_id_range check (from_id >= 0 AND from_id <= 18446744073709551615),
    to_id    numeric(20, 0) not null constraint chk_to_id_range check (to_id >= 0 AND to_id <= 18446744073709551615),
    nickname varchar(255) null,
    type     int          not null,
    constraint IDX_a0b2ff0a598df0b0d055934a17
        unique (from_id, to_id),
    constraint FK_9af4194bab1250b1c584ae4f1d7
        foreign key (from_id) references users (id)
            on delete cascade,
    constraint FK_9c7f6b98a9843b76dce1b0c878b
        foreign key (to_id) references users (id)
            on delete cascade
);
