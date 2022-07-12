-- Your SQL goes here

ALTER TABLE featured_user_communities ADD COLUMN owner_name
VARCHAR(200) NOT NULL;
ALTER TABLE featured_user_communities ADD COLUMN owner_link
VARCHAR(200) NOT NULL;
ALTER TABLE featured_user_communities ADD COLUMN owner_image
VARCHAR(500);

ALTER TABLE news_user_communities ADD COLUMN owner_name
VARCHAR(200) NOT NULL;
ALTER TABLE news_user_communities ADD COLUMN owner_link
VARCHAR(200) NOT NULL;
ALTER TABLE news_user_communities ADD COLUMN owner_image
VARCHAR(500);

ALTER TABLE notify_user_communities ADD COLUMN owner_name
VARCHAR(200) NOT NULL;
ALTER TABLE notify_user_communities ADD COLUMN owner_link
VARCHAR(200) NOT NULL;
ALTER TABLE notify_user_communities ADD COLUMN owner_image
VARCHAR(500);
