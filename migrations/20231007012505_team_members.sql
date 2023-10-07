create table if not exists team_members
(
    id               varchar(255) not null
        primary key,
    membership_state int          not null,
    permissions      text         not null,
    team_id          varchar(255) null,
    user_id          varchar(255) null,
    constraint FK_c2bf4967c8c2a6b845dadfbf3d4
        foreign key (user_id) references users (id)
            on delete cascade,
    constraint FK_fdad7d5768277e60c40e01cdcea
        foreign key (team_id) references teams (id)
            on delete cascade
);