-- Your SQL goes here

-- Категории сообществ -------
CREATE TABLE community_categorys (
    id SERIAL PRIMARY KEY,      -- id объекта
    name VARCHAR(100) NOT NULL, -- название
    avatar VARCHAR(500),        -- аватар
    position SMALLINT NOT NULL  -- порядковый номер
);

-- Суб-категории сообществ -------
CREATE TABLE community_subcategorys (
    id          SERIAL PRIMARY KEY,    -- id объекта
    name        VARCHAR(100) NOT NULL, -- название
    category_id INT NOT NULL,          -- id категории
    avatar      VARCHAR(500),          -- аватар
    position    SMALLINT NOT NULL      -- порядковый номер
);

CREATE TABLE communitys (
    id            SERIAL PRIMARY KEY,     -- id объекта
    name          VARCHAR(100) NOT NULL,  -- название
    description   VARCHAR(500),           -- описание
    status        VARCHAR(100),           -- статус
    types         SMALLINT NOT NULL,      -- тип
    perm          "char" NOT NULL,        -- приватность
    level         SMALLINT NOT NULL DEFAULT 100, -- уровень доверия
    link          VARCHAR(100) NOT NULL,  -- красивая ссылка
    b_avatar      VARCHAR(500),           -- большой аватар
    s_avatar      VARCHAR(500),           -- маленький аватар
    cover         VARCHAR(500),           -- баннер
    category_id   INT NOT NULL,           -- id категории
    user_id       INT NOT NULL,           -- id создателя
    created       TIMESTAMP NOT NULL
);
CREATE INDEX communitys_user_id_idx ON communitys (user_id);

-- Члены сообщества -------
CREATE TABLE communities_memberships (
    id                SERIAL PRIMARY KEY,             -- id объекта
    user_id           INT NOT NULL,                   -- id пользователя
    community_id      INT NOT NULL,                   -- id сообщества
    is_administrator  BOOLEAN NOT NULL DEFAULT false, -- админ?
    is_moderator      BOOLEAN NOT NULL DEFAULT false, -- Модератор?
    is_editor         BOOLEAN NOT NULL DEFAULT false, -- Редактор?
    is_advertiser     BOOLEAN NOT NULL DEFAULT false, -- Рекламщик?
    created           TIMESTAMP NOT NULL,             -- Создано
    visited           INT NOT NULL DEFAULT 0         -- Визиты в сообщество
);
CREATE UNIQUE INDEX communities_memberships_unq ON communities_memberships (user_id, community_id);

CREATE TABLE community_infos (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,

    posts        INT NOT NULL,
    members      INT NOT NULL,
    photos       INT NOT NULL,
    goods        INT NOT NULL,
    tracks       INT NOT NULL,
    videos       INT NOT NULL,
    docs         INT NOT NULL,
    articles     INT NOT NULL,
    survey       INT NOT NULL,
    planners     INT NOT NULL,
    avatar_id    INT
);
CREATE UNIQUE INDEX community_infos_unq ON community_infos (id, community_id);

CREATE TABLE community_privates (
    id               SERIAL PRIMARY KEY,
    community_id     INT NOT NULL,
    can_see_member   "char" NOT NULL, -- Кто видит сообщества
    can_see_info     "char" NOT NULL, -- Кто видит информацию
    can_send_message "char" NOT NULL, -- Кто пишет сообщения
    can_see_post     "char" NOT NULL, -- Кто видит записи
    can_see_photo    "char" NOT NULL, -- Кто видит фотографии
    can_see_good     "char" NOT NULL, -- Кто видит товары
    can_see_video    "char" NOT NULL, -- Кто видит видеозаписи
    can_see_music    "char" NOT NULL, -- Кто видит аудиозапис
    can_see_planner  "char" NOT NULL, -- Кто видит раздел планирования
    can_see_doc      "char" NOT NULL, -- Кто видит документы
    can_see_survey   "char" NOT NULL, -- Кто видит опросы

    can_see_settings "char" NOT NULL, -- Кто видит настройки
    can_see_log      "char" NOT NULL, -- Кто видит логи
    can_see_stat     "char" NOT NULL, -- Кто видит статистику
    can_see_forum    "char" NOT NULL -- Кто видит опросы
);
CREATE UNIQUE INDEX community_privates_unq ON community_privates (id, community_id);

-- Уведомления сообщества -------
CREATE TABLE community_notifications (
    id                   SERIAL PRIMARY KEY,
    community_id         INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    community_invite     BOOLEAN NOT NULL DEFAULT true
);
CREATE UNIQUE INDEX community_notifications_unq ON community_notifications (id, community_id);

-- Черный список -------
CREATE TABLE community_banner_users (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    user_id      INT NOT NULL
);
CREATE UNIQUE INDEX community_banner_users_unq ON community_banner_users (community_id, user_id);
