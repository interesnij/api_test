
CREATE TABLE post_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,

    community_id    INT,
    user_id         INT NOT NULL,
    owner_name      VARCHAR(200) NOT NULL,
    owner_link      VARCHAR(200) NOT NULL,
    owner_image     VARCHAR(500),

    types           SMALLINT NOT NULL,
    description     VARCHAR(500),
    image           VARCHAR(500),
    created         TIMESTAMP NOT NULL,

    count           INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,

    can_see_el      "char" NOT NULL,
    can_see_comment "char" NOT NULL,
    create_el       "char" NOT NULL,
    create_comment  "char" NOT NULL,
    copy_el         "char" NOT NULL,
    reactions       VARCHAR(100)
);
CREATE INDEX post_lists_user_id_idx ON post_lists (user_id);
CREATE INDEX post_lists_community_id_idx ON post_lists (community_id);

CREATE TABLE posts (
    id              SERIAL PRIMARY KEY,
    content         VARCHAR(5000),

    community_id    INT,
    user_id         INT NOT NULL,
    owner_name      VARCHAR(200) NOT NULL,
    owner_link      VARCHAR(200) NOT NULL,
    owner_image     VARCHAR(500),
    post_list_id    INT NOT NULL,

    types           "char" NOT NULL,
    attach          VARCHAR(200),
    comment_enabled BOOLEAN NOT NULL DEFAULT true,
    created         TIMESTAMP NOT NULL,

    comment         INT NOT NULL,
    view            INT NOT NULL,
    repost          INT NOT NULL,
    copy            INT NOT NULL,
    position        SMALLINT NOT NULL,
    is_signature    BOOLEAN NOT NULL DEFAULT false,
    parent_id       INT,
    reactions       INT NOT NULL,

    CONSTRAINT fk_posts_parent
        FOREIGN KEY(parent_id)
            REFERENCES posts(id),

    CONSTRAINT fk_posts_list
        FOREIGN KEY(post_list_id)
            REFERENCES post_lists(id)
);
CREATE INDEX posts_community_id_idx ON posts (community_id);
CREATE INDEX posts_user_id_idx ON posts (user_id);
CREATE INDEX posts_list_id_idx ON posts (post_list_id);
CREATE INDEX posts_parent_id_idx ON posts (parent_id);

CREATE TABLE post_comments (
    id         SERIAL PRIMARY KEY,

    post_id    INT NOT NULL,
    user_id    INT NOT NULL,
    user_name  VARCHAR(200) NOT NULL,
    user_link  VARCHAR(200) NOT NULL,
    user_image VARCHAR(500),
    sticker_id INT,
    parent_id  INT,

    content    VARCHAR(1000),
    attach     VARCHAR(200),
    types      "char" NOT NULL,
    created    TIMESTAMP NOT NULL,
    repost     INT NOT NULL,
    reactions  INT NOT NULL,

    CONSTRAINT fk_post_comment
        FOREIGN KEY(post_id)
            REFERENCES posts(id),

    CONSTRAINT fk_post_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES post_comments(id)
);
CREATE INDEX post_comments_post_id_idx ON post_comments (post_id);
CREATE INDEX post_comments_user_id_idx ON post_comments (user_id);
CREATE INDEX post_comments_sticker_id_idx ON post_comments (sticker_id);
CREATE INDEX post_comments_parent_id_idx ON post_comments (parent_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_post_list_collections (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    post_list_id INT NOT NULL
);
CREATE UNIQUE INDEX user_post_list_collections_unq ON user_post_list_collections (user_id, post_list_id);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_post_list_collections (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    post_list_id INT NOT NULL
);
CREATE UNIQUE INDEX community_post_list_collections_unq ON community_post_list_collections (community_id, post_list_id);

-- включения и исключения для пользователей касательно конкретного списка записей -------
CREATE TABLE post_list_perms (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    post_list_id    INT NOT NULL,
    can_see_item    "char",
    can_see_comment "char",
    create_item     "char",
    create_comment  "char",
    can_copy        "char"
);
CREATE UNIQUE INDEX post_list_perms_unq ON post_list_perms (user_id, post_list_id);

CREATE TABLE posts_perms (
  id                   SERIAL PRIMARY KEY,
  user_id              INT NOT NULL,
  can_see_post         "char",
  can_see_post_comment "char",
  can_copy_post        "char",
  can_work_post        "char"
);
CREATE UNIQUE INDEX posts_perms_unq ON posts_perms (user_id, id);

CREATE TABLE post_reactions (
    id       SERIAL PRIMARY KEY,
    post_id  INT NOT NULL,
    field_1  INT NOT NULL,
    field_2  INT NOT NULL,
    field_3  INT NOT NULL,
    field_4  INT NOT NULL,
    field_5  INT NOT NULL,
    field_6  INT NOT NULL,
    field_7  INT NOT NULL,
    field_8  INT NOT NULL,
    field_9  INT NOT NULL,
    field_10 INT NOT NULL,
    field_11 INT NOT NULL,
    field_12 INT NOT NULL,
    field_13 INT NOT NULL,
    field_14 INT NOT NULL,
    field_15 INT NOT NULL,
    field_16 INT NOT NULL
);
CREATE UNIQUE INDEX post_reactions_unq ON post_reactions (post_id, id);

CREATE TABLE post_comment_reactions (
    id       SERIAL PRIMARY KEY,
    post_comment_id INT NOT NULL,
    field_1  INT NOT NULL,
    field_2  INT NOT NULL,
    field_3  INT NOT NULL,
    field_4  INT NOT NULL,
    field_5  INT NOT NULL,
    field_6  INT NOT NULL,
    field_7  INT NOT NULL,
    field_8  INT NOT NULL,
    field_9  INT NOT NULL,
    field_10 INT NOT NULL,
    field_11 INT NOT NULL,
    field_12 INT NOT NULL,
    field_13 INT NOT NULL,
    field_14 INT NOT NULL,
    field_15 INT NOT NULL,
    field_16 INT NOT NULL
);
CREATE UNIQUE INDEX post_comment_reactions_unq ON post_comment_reactions (post_comment_id, id);

-- Уведомления записей -------
CREATE TABLE user_post_notifications (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    comment         BOOLEAN NOT NULL DEFAULT true,
    comment_reply   BOOLEAN NOT NULL DEFAULT true,
    mention         BOOLEAN NOT NULL DEFAULT true,
    comment_mention BOOLEAN NOT NULL DEFAULT true,
    repost          BOOLEAN NOT NULL DEFAULT true,
    reactions       BOOLEAN NOT NULL DEFAULT true
);
CREATE UNIQUE INDEX user_post_notifications_unq ON user_post_notifications (user_id, id);

-- Уведомления записей -------
CREATE TABLE community_post_notifications (
    id              SERIAL PRIMARY KEY,
    community_id    INT NOT NULL,
    comment         BOOLEAN NOT NULL DEFAULT true,
    comment_reply   BOOLEAN NOT NULL DEFAULT true,
    mention         BOOLEAN NOT NULL DEFAULT true,
    comment_mention BOOLEAN NOT NULL DEFAULT true,
    repost          BOOLEAN NOT NULL DEFAULT true,
    reactions       BOOLEAN NOT NULL DEFAULT true
);
CREATE UNIQUE INDEX community_post_notifications_unq ON community_post_notifications (id, community_id);

-- Порядок следования списка записей -------
CREATE TABLE user_post_list_positions (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,     -- Пользователь
    list_id  INT NOT NULL,     -- Список записей
    position SMALLINT NOT NULL, -- Порядок отображения
    types    "char" NOT NULL     -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списка записей -------
CREATE TABLE community_post_list_positions (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,      -- Сообщество
    list_id      INT NOT NULL,      -- Список записей
    position     SMALLINT NOT NULL,      -- Порядок отображения
    types        "char" NOT NULL       -- 1 - открыт, 0 - недоступен (например, удален)
);
CREATE UNIQUE INDEX community_post_list_positions_unq ON community_post_list_positions (id, community_id);
