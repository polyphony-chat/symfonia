create table if not exists sessions
(
    id          numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615) primary key,
    user_id     numeric(20, 0) null constraint chk_user_id_range check (user_id >= 0 AND user_id <= 18446744073709551615),
    session_id  numeric(20, 0) not null constraint chk_session_id_range check (session_id >= 0 AND session_id <= 18446744073709551615),
    activities  text         null,
    client_info text         not null,
    status      varchar(255) not null,
    constraint FK_085d540d9f418cfbdc7bd55bb19
        foreign key (user_id) references users (id)
            on delete cascade
);
