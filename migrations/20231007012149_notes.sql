CREATE SEQUENCE notes_index_seq;

create table if not exists notes
(
    index     numeric(20, 0) not null default nextval('notes_index_seq') constraint chk_index_range check (index >= 0 and index <= 18446744073709551615) primary key,
    content   varchar(256) not null,
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