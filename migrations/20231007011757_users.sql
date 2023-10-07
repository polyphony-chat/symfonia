create table if not exists users
(
    id                  varchar(255)      not null
        primary key,
    username            varchar(255)      not null,
    discriminator       varchar(255)      not null,
    avatar              varchar(255)      null,
    accent_color        int               null,
    banner              varchar(255)      null,
    theme_colors        text              null,
    pronouns            varchar(255)      null,
    phone               varchar(255)      null,
    desktop             tinyint           not null,
    mobile              tinyint           not null,
    premium             tinyint           not null,
    premium_type        int               not null,
    bot                 tinyint           not null,
    bio                 varchar(255)      not null,
    `system`            tinyint           not null,
    nsfw_allowed        tinyint           not null,
    mfa_enabled         tinyint           not null,
    webauthn_enabled    tinyint default 0 not null,
    totp_secret         varchar(255)      null,
    totp_last_ticket    varchar(255)      null,
    created_at          datetime          not null,
    premium_since       datetime          null,
    verified            tinyint           not null,
    disabled            tinyint           not null,
    deleted             tinyint           not null,
    email               varchar(255)      null,
    flags               int               not null,
    public_flags        int               not null,
    purchased_flags     int               not null,
    premium_usage_flags int               not null,
    rights              bigint            not null,
    data                text              not null,
    fingerprints        text              not null,
    extended_settings   text              not null,
    settingsIndex       int               null,
    constraint REL_0c14beb78d8c5ccba66072adbc
        unique (settingsIndex),
    constraint FK_0c14beb78d8c5ccba66072adbc7
        foreign key (settingsIndex) references user_settings (`index`)
);

