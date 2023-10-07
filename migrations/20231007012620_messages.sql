create table if not exists messages
(
    id                   varchar(255)                             not null
        primary key,
    channel_id           varchar(255)                             null,
    guild_id             varchar(255)                             null,
    author_id            varchar(255)                             null,
    member_id            varchar(255)                             null,
    webhook_id           varchar(255)                             null,
    application_id       varchar(255)                             null,
    content              varchar(255)                             null,
    timestamp            datetime(6) default current_timestamp(6) not null,
    edited_timestamp     datetime                                 null,
    tts                  tinyint                                  null,
    mention_everyone     tinyint                                  null,
    embeds               text                                     not null,
    reactions            text                                     not null,
    nonce                text                                     null,
    pinned               tinyint                                  null,
    type                 int                                      not null,
    activity             text                                     null,
    flags                int                                      null,
    message_reference    text                                     null,
    interaction          text                                     null,
    components           text                                     null,
    message_reference_id varchar(255)                             null,
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