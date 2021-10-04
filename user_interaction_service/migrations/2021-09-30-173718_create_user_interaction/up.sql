-- Your SQL goes here
CREATE TABLE user_interactions(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content_id INTEGER NOT NULL,
    user_read BOOLEAN NOT NULL,
    user_like BOOLEAN NOT NULL,
    CONSTRAINT content_interaction UNIQUE (user_id, content_id)
)
