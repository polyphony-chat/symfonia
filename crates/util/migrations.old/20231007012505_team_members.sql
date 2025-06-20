create table if not exists team_members
(
    id numeric(20, 0) not null constraint chk_id_range check (
        id >= 0 and id <= 18446744073709551615
    ) primary key,
    membership_state int not null,
    permissions text not null,
    team_id numeric(20, 0) null constraint chk_team_id_range check (
        team_id >= 0 and team_id <= 18446744073709551615
    ),
    user_id numeric(20, 0) null constraint chk_user_id_range check (
        user_id >= 0 and user_id <= 18446744073709551615
    ),
    constraint fk_c2bf4967c8c2a6b845dadfbf3d4
    foreign key (user_id) references users (id)
    on delete cascade,
    constraint fk_fdad7d5768277e60c40e01cdcea
    foreign key (team_id) references teams (id)
    on delete cascade
);
