create table if not exists users
(
    id                  numeric(20, 0)                  not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    username            varchar(255)                    not null,
    discriminator       varchar(255) default 0000       not null,
    desktop             boolean default false           not null,
    mobile              boolean default false           not null,
    premium             boolean                         not null,
    premium_type        numeric(5, 0)                   not null constraint chk_premium_type_unsigned check (premium_type >= 0 and premium_type <= 65535),
    bot                 boolean default false           not null,
    bio                 varchar(255)                    not null,
    system              boolean default false           not null,
    nsfw_allowed        boolean default false           not null,
    mfa_enabled         boolean default false           not null,
    webauthn_enabled    boolean default false           not null,
    created_at          timestamp                       not null,
    verified            boolean default false           not null,
    disabled            boolean default false           not null,
    deleted             boolean default false           not null,
    flags               numeric(20, 0)                  not null constraint chk_flags_range check (flags >= 0 AND flags <= 18446744073709551615),
    public_flags        numeric(10, 0)                  not null constraint chk_int_unsigned check (public_flags >= 0 and public_flags <= 4294967295),
    purchased_flags     int                             not null,
    premium_usage_flags int                             not null,
    rights              bigint                          not null,
    data                json                            not null,
    fingerprints        text                            not null,
    extended_settings   text                            not null,
    settings_index      numeric(20, 0)                  not null constraint chk_index_range check (settings_index >= 0 and settings_index <= 18446744073709551615),
    constraint users_settings_index_uindex
        unique (settings_index),
    constraint users_user_settings_index_fk
        foreign key (settings_index) references user_settings (index)
);

