CREATE TABLE accounts (
    id serial primary key,
    email varchar(256) not null unique,
    password varchar(64) not null,
    display_name varchar(16) not null
);