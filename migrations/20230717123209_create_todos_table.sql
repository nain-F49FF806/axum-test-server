-- Add migration script here

CREATE TABLE IF NOT EXISTS todos (
    -- https://stackoverflow.com/questions/20021983/what-is-the-difference-between-serial-and-auto-increment-in-mysql
    -- https://www.sqltutorial.org/sql-identity/
    id SERIAL PRIMARY KEY,
    -- https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-char-varchar-text/
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);
