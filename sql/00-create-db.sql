CREATE TABLE todos (
    id serial primary key,
    title varchar(250) not null,
    description text null,
    completed boolean not null
);

