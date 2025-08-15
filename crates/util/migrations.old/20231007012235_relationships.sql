CREATE SEQUENCE relationships_index_seq;

CREATE TABLE IF NOT EXISTS relationships
(
    index numeric(20, 0) NOT NULL DEFAULT nextval(
        'relationships_index_seq'
    ) CONSTRAINT chk_index_range CHECK (
        index >= 0 AND index <= 18446744073709551615
    ) PRIMARY KEY,
    from_id numeric(20, 0) NOT NULL CONSTRAINT chk_from_id_range CHECK (
        from_id >= 0 AND from_id <= 18446744073709551615
    ),
    to_id numeric(20, 0) NOT NULL CONSTRAINT chk_to_id_range CHECK (
        to_id >= 0 AND to_id <= 18446744073709551615
    ),
    nickname varchar(255) NULL,
    type numeric(3, 0) NOT NULL CONSTRAINT chk_type_range CHECK (
        type >= 0 AND type <= 255
    ),
    since timestamptz NULL DEFAULT now(),
    CONSTRAINT fk_unique_relationships
    UNIQUE (from_id, to_id),
    CONSTRAINT fk_from_id_references_users_id
    FOREIGN KEY (from_id) REFERENCES users (id)
    ON DELETE CASCADE,
    CONSTRAINT fk_to_id_references_users_id
    FOREIGN KEY (to_id) REFERENCES users (id)
    ON DELETE CASCADE
);
