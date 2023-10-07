create table if not exists member_roles
(
    `index` int          not null,
    role_id varchar(255) not null,
    primary key (`index`, role_id),
    constraint FK_5d7ddc8a5f9c167f548625e772e
        foreign key (`index`) references members (`index`)
            on update cascade on delete cascade,
    constraint FK_e9080e7a7997a0170026d5139c1
        foreign key (role_id) references roles (id)
            on update cascade on delete cascade
);

create index if not exists IDX_5d7ddc8a5f9c167f548625e772
    on member_roles (`index`);

create index if not exists IDX_e9080e7a7997a0170026d5139c
    on member_roles (role_id);