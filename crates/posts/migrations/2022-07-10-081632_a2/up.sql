-- Your SQL goes here

ALTER TABLE post_reposts ALTER COLUMN post_id
SET NOT NULL; 
ALTER TABLE post_reposts ALTER COLUMN message_id
SET NOT NULL; 
