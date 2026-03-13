CREATE SCHEMA IF NOT EXISTS todo;

CREATE TABLE IF NOT EXISTS todo.todo
(
    id          uuid    NOT NULL,
    owner       varchar NOT NULL,
    title       varchar NOT NULL,
    description varchar NOT NULL,
    status      varchar NOT NULL,
    created     timestamp with time zone,
    updated     timestamp with time zone,
    PRIMARY KEY (id)
);

CREATE SEQUENCE IF NOT EXISTS todo.todo_seq
    INCREMENT 1
    START 1;