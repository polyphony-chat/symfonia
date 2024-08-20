create table if not exists applications
(
    id                             varchar(255) not null
        primary key,
    name                           varchar(255) not null,
    icon                           varchar(255) null,
    description                    varchar(255) null,
    summary                        varchar(255) null,
    type                           text         null,
    hook                           smallint      not null,
    bot_public                     smallint      not null,
    bot_require_code_grant         smallint      not null,
    verify_key                     varchar(255) not null,
    flags                          int          not null,
    redirect_uris                  text         null,
    rpc_application_state          int          null,
    store_application_state        int          null,
    verification_state             int          null,
    interactions_endpoint_url      varchar(255) null,
    integration_public             smallint      null,
    integration_require_code_grant smallint      null,
    discoverability_state          int          null,
    discovery_eligibility_flags    int          null,
    tags                           text         null,
    cover_image                    varchar(255) null,
    install_params                 text         null,
    terms_of_service_url           varchar(255) null,
    privacy_policy_url             varchar(255) null,
    owner_id                       varchar(255) null,
    bot_user_id                    varchar(255) null,
    team_id                        varchar(255) null,
    constraint REL_2ce5a55796fe4c2f77ece57a64
        unique (bot_user_id),
    constraint FK_2ce5a55796fe4c2f77ece57a647
        foreign key (bot_user_id) references users (id)
            on delete cascade,
    constraint FK_a36ed02953077f408d0f3ebc424
        foreign key (team_id) references teams (id)
            on delete cascade,
    constraint FK_e57508958bf92b9d9d25231b5e8
        foreign key (owner_id) references users (id)
            on delete cascade
);