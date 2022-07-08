-- Your SQL goes here

CREATE TABLE custom_links (
    id   SERIAL PRIMARY KEY,
    link VARCHAR(100) NOT NULL,
    owner SMALLINT NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX custom_links_unq ON custom_links (link);

CREATE TABLE sticker_categories (
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(100) NOT NULL,
    position     SMALLINT NOT NULL DEFAULT 0,
    user_id      INT NOT NULL,
    community_id INT,
    owner_name   VARCHAR(200) NOT NULL,
    owner_link   VARCHAR(200) NOT NULL,
    owner_image  VARCHAR(500),
    description  VARCHAR(200),
    avatar       VARCHAR(500)
);
CREATE INDEX sticker_categories_user_id_idx ON sticker_categories (user_id);
CREATE INDEX sticker_categories_community_id_idx ON sticker_categories (community_id);

CREATE TABLE stickers (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    category_id INT NOT NULL,
    image       VARCHAR(500) NOT NULL,

    CONSTRAINT fk_stickers
        FOREIGN KEY(category_id)
            REFERENCES sticker_categories(id)
);
CREATE INDEX stickers_category_id_idx ON stickers (category_id);

CREATE TABLE smile_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    description VARCHAR(200)
);

CREATE TABLE smiles (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    category_id INT NOT NULL,
    image       VARCHAR(500) NOT NULL,

    CONSTRAINT fk_smiles
        FOREIGN KEY(category_id)
            REFERENCES smile_categories(id)
);
CREATE INDEX smiles_category_id_idx ON smiles (category_id);

CREATE TABLE folders (
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    parent_id    INT,
    owner_name   VARCHAR(200) NOT NULL,
    owner_link   VARCHAR(200) NOT NULL,
    owner_image  VARCHAR(500),

    count        INT NOT NULL,
    repost       INT NOT NULL,
    copy         INT NOT NULL,
    position     SMALLINT NOT NULL,

    can_see_el   "char" NOT NULL,
    create_el    "char" NOT NULL,
    copy_el      "char" NOT NULL,

    CONSTRAINT fk_folders_parent
        FOREIGN KEY(parent_id)
            REFERENCES folders(id)
);

CREATE INDEX folders_user_id_idx ON folders (user_id);
CREATE INDEX folders_community_id_idx ON folders (community_id);
CREATE INDEX folders_parent_id_idx ON folders (parent_id);

CREATE TABLE folder_items (
    id        SERIAL PRIMARY KEY,
    folder_id INT NOT NULL,
    types     SMALLINT NOT NULL,
    object_id INT NOT NULL,
    position  SMALLINT NOT NULL
);
CREATE UNIQUE INDEX folder_items_unq ON folder_items (folder_id, id);
