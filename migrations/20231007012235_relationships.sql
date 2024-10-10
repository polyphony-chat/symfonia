CREATE SEQUENCE relationships_index_seq;

CREATE TABLE IF NOT EXISTS relationships
(
    index    numeric(20, 0) not null default nextval('relationships_index_seq') constraint chk_index_range check (index >= 0 AND index <= 18446744073709551615) primary key,
    from_id  numeric(20, 0) not null constraint chk_from_id_range check (from_id >= 0 AND from_id <= 18446744073709551615),
    to_id    numeric(20, 0) not null constraint chk_to_id_range check (to_id >= 0 AND to_id <= 18446744073709551615),
    nickname varchar(255) null,
    type     numeric(3, 0)  not null constraint chk_type_range check (type >= 0 AND type <= 255),
    since    timestamptz null default now(),
    constraint FK_unique_relationships
        unique (from_id, to_id),
    constraint FK_from_id_references_users_id
        foreign key (from_id) references users (id)
            on delete cascade,
    constraint FK_to_id_references_users_id
        foreign key (to_id) references users (id)
            on delete cascade
);
