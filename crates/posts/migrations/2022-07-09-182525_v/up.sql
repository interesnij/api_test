-- Your SQL goes here

CREATE TABLE post_votes (
  id          SERIAL PRIMARY KEY,
  vote        SMALLINT NOT NULL,
  user_id     INT NOT NULL,
  post_id     INT NOT NULL,
  reaction    SMALLINT NOT NULL -- тип реакции для скорости работы
);
CREATE UNIQUE INDEX post_votes_unq ON post_votes (user_id, post_id);

CREATE TABLE post_comment_votes (
  id              SERIAL PRIMARY KEY,
  vote            SMALLINT NOT NULL,
  user_id         INT NOT NULL,
  post_comment_id INT NOT NULL,
  reaction        SMALLINT NOT NULL
);
CREATE UNIQUE INDEX post_comment_votes_unq ON post_comment_votes (user_id, post_comment_id);
