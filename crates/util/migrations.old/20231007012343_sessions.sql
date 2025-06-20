CREATE SEQUENCE sessions_index_seq;

CREATE TABLE IF NOT EXISTS sessions
(
    index numeric(20, 0) NOT NULL DEFAULT nextval(
        'user_settings_index_seq'
    ) CONSTRAINT chk_index_range CHECK (
        index >= 0 AND index <= 18446744073709551615
    ) PRIMARY KEY,
    user_id numeric(20, 0) NULL CONSTRAINT chk_user_id_range CHECK (
        user_id >= 0 AND user_id <= 18446744073709551615
    ),
    session_id numeric(20, 0) NOT NULL CONSTRAINT chk_session_id_range CHECK (
        session_id >= 0 AND session_id <= 18446744073709551615
    ),
    activities text NULL,
    client_info text NOT NULL,
    status varchar(255) NOT NULL,
    CONSTRAINT fk_085d540d9f418cfbdc7bd55bb19
    FOREIGN KEY (user_id) REFERENCES users (id)
    ON DELETE CASCADE
);
