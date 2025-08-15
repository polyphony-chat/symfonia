CREATE SEQUENCE notes_index_seq;

CREATE TABLE IF NOT EXISTS notes
(
    index numeric(20, 0) NOT NULL DEFAULT nextval(
        'notes_index_seq'
    ) CONSTRAINT chk_index_range CHECK (
        index >= 0 AND index <= 18446744073709551615
    ) PRIMARY KEY,
    content varchar(256) NOT NULL,
    author_id numeric(20, 0) NULL CONSTRAINT chk_author_id_range CHECK (
        author_id >= 0 AND author_id <= 18446744073709551615
    ),
    target_id numeric(20, 0) NULL CONSTRAINT chk_target_id_range CHECK (
        target_id >= 0 AND target_id <= 18446744073709551615
    ),
    CONSTRAINT idx_74e6689b9568cc965b8bfc9150
    UNIQUE (author_id, target_id),
    CONSTRAINT fk_23e08e5b4481711d573e1abecdc
    FOREIGN KEY (target_id) REFERENCES users (id)
    ON DELETE CASCADE,
    CONSTRAINT fk_f9e103f8ae67cb1787063597925
    FOREIGN KEY (author_id) REFERENCES users (id)
    ON DELETE CASCADE
);
