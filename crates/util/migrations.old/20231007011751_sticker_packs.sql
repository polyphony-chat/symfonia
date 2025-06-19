create table if not exists sticker_packs
(
    id numeric(20, 0) not null constraint chk_id_range check (
        id >= 0 and id <= 18446744073709551615
    ) primary key,
    name varchar(255) not null,
    description varchar(255) null,
    banner_asset_id numeric(
        20, 0
    ) null constraint chk_banner_asset_id_range check (
        banner_asset_id >= 0 and banner_asset_id <= 18446744073709551615
    ),
    cover_sticker_id numeric(
        20, 0
    ) null constraint chk_cover_sticker_id_range check (
        cover_sticker_id >= 0 and cover_sticker_id <= 18446744073709551615
    ),
    coverstickerid numeric(
        20, 0
    ) null constraint chk_coverstickerid_range check (
        coverstickerid >= 0 and coverstickerid <= 18446744073709551615
    )
);
