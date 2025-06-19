CREATE SEQUENCE member_roles_index_seq;

CREATE TABLE IF NOT EXISTS member_roles
(
    index numeric(20, 0) NOT NULL CONSTRAINT chk_index_range CHECK (
        index >= 0 AND index <= 18446744073709551615
    ) UNIQUE,
    role_id numeric(20, 0) NOT NULL CONSTRAINT chk_role_id_range CHECK (
        role_id >= 0 AND role_id <= 18446744073709551615
    ),
    PRIMARY KEY (index, role_id),
    CONSTRAINT fk_5d7ddc8a5f9c167f548625e772e
    FOREIGN KEY (index) REFERENCES members (index)
    ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fk_e9080e7a7997a0170026d5139c1
    FOREIGN KEY (role_id) REFERENCES roles (id)
    ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_e9080e7a7997a0170026d5139c
ON member_roles (role_id);

ALTER SEQUENCE member_roles_index_seq OWNED BY member_roles.index;
