create sequence ticket_nr
    as integer
    minvalue 0;

alter sequence ticket_nr owner to ticket;

create sequence user_nr
    as integer
    minvalue 0;

alter sequence user_nr owner to ticket;

create type priority as enum ('low', 'normal', 'high', 'critical');

alter type priority owner to ticket;

create type status as enum ('new', 'in progress', 'done', 'cancelled', 'wont fix', 'duplicate', 'stale', 'resolved');

alter type status owner to ticket;

create table if not exists public.users
(
    user_id      integer default nextval('user_nr'::regclass) not null
        constraint users_pk
            primary key,
    username     text                                         not null,
    display_name text,
    sub          text                                         not null
        constraint users_pk_2
            unique
);

alter table public.users
    owner to ticket;



alter table users
    owner to ticket;

create table if not exists tickets
(
    ticket_id  integer   default nextval('ticket_nr'::regclass) not null
        constraint tickets_pk
            primary key,
    date       timestamp default now()                          not null,
    registrant integer                                          not null
        constraint registrant
            references users,
    title      text                                             not null
);

alter table tickets
    owner to ticket;

create table if not exists ticket_update
(
    ticket_id  integer                              not null
        constraint ticket
            references tickets,
    date       timestamp default now()              not null,
    priority   priority  default 'normal'::priority not null,
    status     status    default 'new'::status      not null,
    title      text                                 not null,
    registrant integer                              not null
        constraint registrant
            references users,
    assignee   integer
        constraint assignee
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

create view ticket (ticket_id, date, modified, title, priority, registrant, status, assignee) as
SELECT tickets.ticket_id,
       tickets.date,
       alt.modified,
       tickets.title,
       COALESCE(alt.priority, 'normal'::priority)   AS priority,
       COALESCE(alt.registrant, tickets.registrant) AS registrant,
       COALESCE(alt.status, 'new'::status)          AS status,
       alt.assignee
FROM (SELECT DISTINCT ON (ticket_update.ticket_id) ticket_update.ticket_id AS ticket,
                                                   ticket_update.date      AS modified,
                                                   ticket_update.ticket_id,
                                                   ticket_update.date,
                                                   ticket_update.priority,
                                                   ticket_update.status,
                                                   ticket_update.title,
                                                   ticket_update.registrant,
                                                   ticket_update.assignee
      FROM ticket_update
      ORDER BY ticket_update.ticket_id, ticket_update.date DESC) alt
         JOIN tickets ON tickets.ticket_id = alt.ticket
LIMIT 10000;

alter table ticket
    owner to ticket;


create function insert_ticket_update(p_title text, p_registrant integer, p_priority priority DEFAULT 'normal'::priority) returns ticket
    language sql
as
$$
    WITH new_ticket AS (
        INSERT INTO tickets
            (title, registrant)
        VALUES
            (p_title, p_registrant)
        RETURNING *
    )
    INSERT INTO ticket_update
        (ticket_id, date, priority, registrant, title)
    SELECT
        new_ticket.ticket_id,
        new_ticket.date,
        p_priority,
        new_ticket.registrant,
        new_ticket.title
    FROM new_ticket
    RETURNING
        ticket_id,
        date,
        date,
        title,
        priority,
        registrant,
        status,
        assignee;
$$;

alter function insert_ticket_update(text, integer, priority) owner to ticket;

