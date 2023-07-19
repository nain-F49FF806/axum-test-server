-- Add migration script here

CREATE TABLE IF NOT EXISTS forward_raw (
    -- https://stackoverflow.com/questions/20021983/what-is-the-difference-between-serial-and-auto-increment-in-mysql
    -- https://www.sqltutorial.org/sql-identity/
    id SERIAL PRIMARY KEY,
    -- https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-char-varchar-text/
    recipient_key TEXT NOT NULL,
    msg MEDIUMBLOB NOT NULL,
    received BOOLEAN NOT NULL DEFAULT FALSE
);
