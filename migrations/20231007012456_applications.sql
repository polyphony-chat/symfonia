create table if not exists applications
(
    id                             numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    name                           varchar(255) not null,
    icon                           varchar(255) null,
    description                    varchar(255) null,
    summary                        varchar(255) null,
    type                           text         null,
    hook                           boolean       not null,
    bot_public                     boolean       not null,
    bot_require_code_grant         boolean       not null,
    verify_key                     numeric(20, 0) not null constraint chk_verify_key_range check (verify_key >= 0 AND verify_key <= 18446744073709551615),
    flags                          int          not null,
    redirect_uris                  text         null,
    rpc_application_state          int          null,
    store_application_state        int          null,
    verification_state             int          null,
    interactions_endpoint_url      varchar(255) null,
    integration_public             boolean       null,
    integration_require_code_grant boolean       null,
    discoverability_state          int          null,
    discovery_eligibility_flags    int          null,
    tags                           text         null,
    cover_image                    varchar(255) null,
    install_params                 text         null,
    terms_of_service_url           varchar(255) null,
    privacy_policy_url             varchar(255) null,
    owner_id                       numeric(20, 0) null constraint chk_owner_id_range check (owner_id >= 0 AND owner_id <= 18446744073709551615),
    bot_user_id                    numeric(20, 0) null constraint chk_bot_user_id_range check (bot_user_id >= 0 AND bot_user_id <= 18446744073709551615),
    team_id                        numeric(20, 0) null constraint chk_team_id_range check (team_id >= 0 AND team_id <= 18446744073709551615),
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
