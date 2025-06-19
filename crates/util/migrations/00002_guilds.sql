CREATE TABLE IF NOT EXISTS guilds (
    id varchar(64) PRIMARY KEY NOT NULL,
    display_name varchar(64) NOT NULL,
    description varchar(1000) NULL,
    icon varchar(255) NULL,
    owner_id varchar(255) null,
    CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) references users (local_name),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);