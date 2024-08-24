create table if not exists categories
(
    id            numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    name          varchar(255) null,
    localizations text         not null,
    is_primary    boolean      null
);
