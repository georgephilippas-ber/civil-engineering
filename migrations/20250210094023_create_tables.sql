-- Add migration script here
drop table if exists survey_data;
drop table if exists survey;
drop table if exists address;

create table address
(
    id     serial primary key,
    street varchar(256),
    number integer,
    city   varchar(128) not null
);

create table survey
(
    id         serial primary key,
    name       varchar(256),
    address_id integer not null references address (id)
);

create table survey_data
(
    id        serial primary key,
    survey_id integer not null references survey (id),
    location  geometry(Point, 2100),
    elevation double precision
);
