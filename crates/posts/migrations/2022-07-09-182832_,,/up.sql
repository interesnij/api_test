-- Your SQL goes here

CREATE TABLE post_list_reposts (
  id           SERIAL PRIMARY KEY,
  post_list_id INT NOT NULL,
  post_id      INT,
  message_id   INT,

  CONSTRAINT fk_post_list_reposts_list
      FOREIGN KEY(post_list_id)
          REFERENCES post_lists(id),

  CONSTRAINT fk_post_list_reposts_post
      FOREIGN KEY(post_id)
          REFERENCES posts(id)
);
CREATE INDEX post_list_reposts_post_list_id_idx ON post_list_reposts (post_list_id);
CREATE INDEX post_list_reposts_post_id_idx ON post_list_reposts (post_id);
CREATE INDEX post_list_reposts_message_id_idx ON post_list_reposts (message_id);
