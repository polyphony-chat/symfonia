alter table read_states
    add constraint read_states_messages_id_fk
        foreign key (last_message_id) references messages (id)
            on delete cascade;
alter table read_states
    add constraint read_states_users_id_fk_2
        foreign key (notifications_cursor) references messages (id)
            on delete cascade;
