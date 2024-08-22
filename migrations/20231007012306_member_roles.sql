CREATE SEQUENCE member_roles_index_seq;

create table if not exists member_roles
(
    index   numeric(20, 0) not null default nextval('member_roles_index_seq') constraint chk_index_range check (index >= 0 and index <= 18446744073709551615) unique,
    role_id varchar(255) not null,
    primary key (index, role_id),
    constraint FK_5d7ddc8a5f9c167f548625e772e
        foreign key (index) references members (index)
            on update cascade on delete cascade,
    constraint FK_e9080e7a7997a0170026d5139c1
        foreign key (role_id) references roles (id)
            on update cascade on delete cascade
);

create index if not exists IDX_e9080e7a7997a0170026d5139c
    on member_roles (role_id);

ALTER SEQUENCE member_roles_index_seq OWNED BY member_roles.index;