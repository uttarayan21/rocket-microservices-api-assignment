-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email_id VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL
)
