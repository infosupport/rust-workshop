CREATE TABLE tasks (
    id serial primary key,
    title varchar(250) not null,
    description text null,
    user_id integer not null,
    completed boolean not null,
    date_created timestamp without time zone not null,
    date_modified timestamp without time zone null
);

