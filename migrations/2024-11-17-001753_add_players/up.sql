-- Your SQL goes here

CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(255) NOT NULL,
    fist_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL
)