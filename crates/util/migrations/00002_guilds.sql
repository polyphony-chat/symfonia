CREATE TABLE IF NOT EXISTS guilds (
    id varchar(64) PRIMARY KEY NOT NULL,
    display_name varchar(64) NOT NULL,
    description varchar(1000) NULL,
    icon varchar(255) NULL,
    owner_id varchar(255) NULL,
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES users (local_name),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);
