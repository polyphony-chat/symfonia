create table if not exists client_release
(
    id       numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    name     varchar(255) not null,
    pub_date timestamp     not null,
    url      varchar(255) not null,
    platform varchar(255) not null,
    enabled  boolean      not null,
    notes    varchar(255) null
);
