-- Your SQL goes here
CREATE TABLE contents(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    story VARCHAR NOT NULL,
    published TIMESTAMPTZ NOT NULL,
    user_id INTEGER NOT NULL
);
SET TIMEZONE to 'GMT';
