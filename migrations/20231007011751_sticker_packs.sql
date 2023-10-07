create table if not exists sticker_packs
(
    id               varchar(255) not null
        primary key,
    name             varchar(255) not null,
    description      varchar(255) null,
    banner_asset_id  varchar(255) null,
    cover_sticker_id varchar(255) null,
    coverStickerId   varchar(255) null
);