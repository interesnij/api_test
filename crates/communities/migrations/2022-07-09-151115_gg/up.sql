-- Your SQL goes here

CREATE TABLE community_follows (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    community_id INT NOT NULL,
    view         BOOLEAN NOT NULL DEFAULT false,
    visited      INT NOT NULL
);
CREATE UNIQUE INDEX follows_community_user_unq ON community_follows (user_id, community_id);
