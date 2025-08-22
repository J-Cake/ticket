create type priority as enum ('low', 'normal', 'high', 'critical');

alter type priority owner to ticket;

create type status as enum ('new', 'in progress', 'done', 'cancelled', 'wontfix', 'duplicate', 'stale', 'resolved');

alter type status owner to ticket;

create table if not exists users
(
    user_id      integer not null
        constraint users_pk
            primary key,
    username     text    not null,
    display_name text
);

alter table users
    owner to ticket;

create table if not exists tickets
(
    ticket_id  integer                 not null
        constraint tickets_pk
            primary key,
    date       timestamp default now() not null,
    registrant integer                 not null
        constraint registrant
            references users,
    title      text                    not null
);

alter table tickets
    owner to ticket;

create table if not exists ticket_update
(
    ticket_id  integer                 not null
        constraint ticket
            references tickets,
    date       timestamp default now() not null,
    priority   priority,
    status     status,
    title      text,
    registrant integer
        constraint registrant
            references users
);

alter table ticket_update
    owner to ticket;

create table if not exists comments
(
    ticket  integer                 not null
        constraint ticket
            references tickets,
    date    timestamp default now() not null,
    comment text
);

alter table comments
    owner to ticket;

