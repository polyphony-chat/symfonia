create table if not exists bans
(
    id          numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    user_id     numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    guild_id    numeric(20, 0) null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    executor_id numeric(20, 0) null constraint chk_executor_id check (executor_id >= 0 AND executor_id <= 18446744073709551615),
    ip          varchar(255) not null,
    reason      varchar(255) null,
    constraint FK_07ad88c86d1f290d46748410d58
        foreign key (executor_id) references users (id),
    constraint FK_5999e8e449f80a236ff72023559
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_9d3ab7dd180ebdd245cdb66ecad
        foreign key (guild_id) references guilds (id)
            on delete cascade
); 