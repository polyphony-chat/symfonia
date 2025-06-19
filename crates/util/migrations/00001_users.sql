CREATE TABLE IF NOT EXISTS users (
    local_name text not null primary key,
    display_name text null,
    pronouns varchar(32) null,
    about varchar(1000) null,
    avatar varchar(255) null,
    availability smallint not null default 0 constraint chk_availability check (
        availability >= 0
        and availability <= 3
    ),
    status varchar (50) null,
    timezone text references tzone(tzone_name) null
);