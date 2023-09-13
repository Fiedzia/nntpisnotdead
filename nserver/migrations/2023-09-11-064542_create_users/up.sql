-- Your SQL goes here
create table users(
    id serial primary key,
    name varchar not null,
    email varchar not null
);

create index devorum_users_name_idx on users(name);
