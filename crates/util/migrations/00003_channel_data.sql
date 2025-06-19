CREATE TABLE IF NOT EXISTS channel_data (
    id bigserial PRIMARY KEY,
    type smallint not null default 0 constraint chk_type check (
        type >= 0
        and type <= 3
    ),
    name varchar(64) NOT NULL,
    description text NULL
);