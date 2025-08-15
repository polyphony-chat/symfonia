CREATE TABLE IF NOT EXISTS users (
    local_name text NOT NULL PRIMARY KEY,
    display_name text NULL,
    pronouns varchar(32) NULL,
    about varchar(1000) NULL,
    avatar varchar(255) NULL,
    availability smallint NOT NULL DEFAULT 0 CONSTRAINT chk_availability CHECK (
        availability >= 0
        AND availability <= 3
    ),
    status varchar(50) NULL,
    timezone text REFERENCES tzone (tzone_name) NULL
);
