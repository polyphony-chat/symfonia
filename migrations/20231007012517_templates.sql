create table if not exists templates
(
    id                      varchar(255) not null
        primary key,
    code                    varchar(255) not null,
    name                    varchar(255) not null,
    description             varchar(255) null,
    usage_count             int          null,
    creator_id              varchar(255) null,
    created_at              datetime     not null,
    updated_at              datetime     not null,
    source_guild_id         varchar(255) null,
    serialized_source_guild text         not null,
    constraint IDX_be38737bf339baf63b1daeffb5
        unique (code),
    constraint FK_445d00eaaea0e60a017a5ed0c11
        foreign key (source_guild_id) references guilds (id)
            on delete cascade,
    constraint FK_d7374b7f8f5fbfdececa4fb62e1
        foreign key (creator_id) references users (id)
);

alter table guilds
    add constraint FK_e2a2f873a64a5cf62526de42325
        foreign key (template_id) references templates (id);