CREATE TABLE users(
  id serial primary key,
  email_address varchar(250) not null,
  api_key varchar(500) not null,
  date_created timestamp without time zone not null,
  date_modified timestamp without time zone null
);
