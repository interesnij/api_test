-- Your SQL goes here

CREATE TABLE reactions (
  id        SERIAL PRIMARY KEY,
  types     SMALLINT NOT NULL,
  image     VARCHAR(500) NOT NULL,
  gif       VARCHAR(500) NOT NULL,
  name      VARCHAR(100) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true,
  position  SMALLINT NOT NULL
);
