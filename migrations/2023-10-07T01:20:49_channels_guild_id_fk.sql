alter table channels
    add constraint FK_c253dafe5f3a03ec00cd8fb4581
        foreign key (guild_id) references guilds (id)
            on delete cascade;