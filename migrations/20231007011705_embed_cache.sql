create table if not exists embed_cache
(
    id    numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    url   varchar(255) not null,
    embed text         not null
);
