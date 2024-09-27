create table if not exists rate_limits
(
    id          numeric(20, 0)  not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    executor_id numeric(20, 0)  not null constraint chk_executor_id_range check (executor_id >= 0 AND executor_id <= 18446744073709551615),
    hits        int             not null,
    blocked     boolean         not null,
    expires_at  timestamp       not null
);
