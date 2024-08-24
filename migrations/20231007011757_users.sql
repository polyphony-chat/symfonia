create table if not exists users
(
    id                  numeric(20, 0)     not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    username            varchar(255)       not null,
    discriminator       varchar(255)       not null,
    avatar              varchar(255)       null,
    accent_color        int                null,
    banner              varchar(255)       null,
    theme_colors        text               null,
    pronouns            varchar(255)       null,
    phone               varchar(255)       null,
    desktop             smallint           not null,
    mobile              smallint           not null,
    premium             smallint           not null,
    premium_type        numeric(5, 0)      not null constraint chk_smallint_unsigned check (premium_type >= 0 and premium_type <= 65535),
    bot                 smallint           not null,
    bio                 varchar(255)       not null,
    system            smallint           not null,
    nsfw_allowed        smallint           not null,
    mfa_enabled         smallint           not null,
    webauthn_enabled    smallint default 0 not null,
    totp_secret         varchar(255)       null,
    totp_last_ticket    varchar(255)       null,
    created_at          timestamp          not null,
    premium_since       timestamp          null,
    verified            smallint           not null,
    disabled            smallint           not null,
    deleted             smallint           not null,
    email               varchar(255)       null,
    flags               numeric(20, 0)     not null constraint chk_flags_range check (flags >= 0 AND flags <= 18446744073709551615),
    public_flags        numeric(10, 0)     not null constraint chk_int_unsigned check (public_flags >= 0 and public_flags <= 4294967295),
    purchased_flags     int                not null,
    premium_usage_flags int                not null,
    rights              bigint             not null,
    data                text               not null,
    fingerprints        text               not null,
    extended_settings   text               not null,
    settingsIndex       numeric(20, 0)     null constraint chk_settingsIndex_range check (settingsIndex >= 0 AND settingsIndex <= 18446744073709551615),
    constraint users_settingsIndex_uindex
        unique (settingsIndex),
    constraint users_user_settings_index_fk
        foreign key (settingsIndex) references user_settings (index)
);

