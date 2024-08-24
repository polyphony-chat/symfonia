create table if not exists messages
(
    id                   numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    channel_id           numeric(20, 0) null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    guild_id             numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    author_id            numeric(20, 0) null constraint chk_author_id_range check (author_id >= 0 AND author_id <= 18446744073709551615),
    member_id            numeric(20, 0) null constraint chk_member_id_range check (member_id >= 0 AND member_id <= 18446744073709551615),
    webhook_id           numeric(20, 0) null constraint chk_webhook_id_range check (webhook_id >= 0 AND webhook_id <= 18446744073709551615),
    application_id       numeric(20, 0) null constraint chk_application_id_range check (application_id >= 0 AND application_id <= 18446744073709551615),
    content              varchar(255) null,
    timestamp            timestamp(6) default current_timestamp(6) not null,
    edited_timestamp     timestamp null,
    tts                  boolean  null,
    mention_everyone     boolean  null,
    embeds               text not null,
    reactions            text not null,
    nonce                text null,
    pinned               boolean  null,
    type                 int not null,
    activity             text null,
    flags                int null,
    message_reference    text null,
    interaction          text null,
    components           text null,
    message_reference_id numeric(20, 0) null constraint chk_message_reference_id_range check (message_reference_id >= 0 AND message_reference_id <= 18446744073709551615),
    constraint IDX_3ed7a60fb7dbe04e1ba9332a8b
        unique (channel_id, id),
    constraint FK_05535bc695e9f7ee104616459d3
        foreign key (author_id) references users (id)
            on delete cascade,
    constraint FK_5d3ec1cb962de6488637fd779d6
        foreign key (application_id) references applications (id),
    constraint FK_61a92bb65b302a76d9c1fcd3174
        foreign key (message_reference_id) references messages (id),
    constraint FK_86b9109b155eb70c0a2ca3b4b6d
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint FK_b0525304f2262b7014245351c76
        foreign key (member_id) references users (id)
            on delete cascade,
    constraint FK_b193588441b085352a4c0109423
        foreign key (guild_id) references guilds (id)
            on delete cascade,
    constraint FK_f83c04bcf1df4e5c0e7a52ed348
        foreign key (webhook_id) references webhooks (id)
);
