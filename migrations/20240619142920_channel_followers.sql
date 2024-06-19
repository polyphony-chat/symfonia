create table channel_followers
(
    webhook_id varchar(255) not null,
    channel_id varchar(255) not null,
    primary key (channel_id, webhook_id),
    constraint channel_followers_channels_id_fk
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint channel_followers_webhooks_id_fk
        foreign key (webhook_id) references webhooks (id)
            on delete cascade
);

