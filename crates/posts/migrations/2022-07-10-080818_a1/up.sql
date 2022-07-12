-- Your SQL goes here

CREATE TABLE post_reposts (
  id           SERIAL PRIMARY KEY,
  post_id      INT,
  message_id   INT, 

  CONSTRAINT fk_post__reposts_post
      FOREIGN KEY(post_id)
          REFERENCES posts(id)
);
CREATE INDEX post_list_reposts_post_idx ON post_reposts (post_id);
CREATE INDEX post_list_reposts_message_idx ON post_reposts (message_id);
