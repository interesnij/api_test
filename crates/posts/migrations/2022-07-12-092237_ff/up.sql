-- Your SQL goes here

ALTER TABLE post_comments RENAME user_name TO owner_name;
ALTER TABLE post_comments RENAME user_link TO owner_link;
ALTER TABLE post_comments RENAME user_image TO owner_image;
