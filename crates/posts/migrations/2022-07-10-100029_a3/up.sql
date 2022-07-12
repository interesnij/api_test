-- Your SQL goes here

ALTER TABLE post_comment_votes ADD COLUMN owner_name
VARCHAR(200) NOT NULL;
ALTER TABLE post_comment_votes ADD COLUMN owner_link
VARCHAR(200) NOT NULL; 
ALTER TABLE post_comment_votes ADD COLUMN owner_image
VARCHAR(500);

ALTER TABLE post_votes ADD COLUMN owner_name
VARCHAR(200) NOT NULL;
ALTER TABLE post_votes ADD COLUMN owner_link
VARCHAR(200) NOT NULL; 
ALTER TABLE post_votes ADD COLUMN owner_image
VARCHAR(500); 
