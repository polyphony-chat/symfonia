create table channel_followers
(
    webhook_id numeric(20, 0) not null constraint chk_webhook_id_range check (webhook_id >= 0 AND webhook_id <= 18446744073709551615),
    channel_id numeric(20, 0) not null constraint chk_channel_id_range check (channel_id >= 0 AND channel_id <= 18446744073709551615),
    primary key (channel_id, webhook_id),
    constraint channel_followers_channels_id_fk
        foreign key (channel_id) references channels (id)
            on delete cascade,
    constraint channel_followers_webhooks_id_fk
        foreign key (webhook_id) references webhooks (id)
            on delete cascade
);
