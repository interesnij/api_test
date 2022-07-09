-- Your SQL goes here

CREATE TABLE phone_codes (
    id    SERIAL PRIMARY KEY,
    phone VARCHAR(14) NOT NULL,
    code  INT NOT NULL
);
