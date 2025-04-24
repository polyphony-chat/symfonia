create table if not exists audit_logs
(
    id          numeric(20, 0)  not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    user_id     numeric(20, 0)  null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    guild_id    numeric(20, 0)  not null constraint chk_guild_id_range check (guild_id >= 0 AND guild_id <= 18446744073709551615),
    action_type int             not null,
    options     text            null,
    changes     text            not null,
    reason      varchar(255)    null,
    target_id   numeric(20, 0)  null constraint chk_target_id_range check (target_id >= 0 AND target_id <= 18446744073709551615),
    constraint FK_3cd01cd3ae7aab010310d96ac8e
        foreign key (target_id) references users (id),
    constraint FK_bd2726fd31b35443f2245b93ba0
        foreign key (user_id) references users (id)
);