-- Your SQL goes here

-- пользователи -------

CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    phone         VARCHAR(14) NOT NULL,
    types         SMALLINT NOT NULL,
    gender        "char" NOT NULL,
    device        "char" NOT NULL,
    language      "char" NOT NULL,
    perm          SMALLINT NOT NULL,
    level         SMALLINT NOT NULL DEFAULT 100,
    password      VARCHAR(500) NOT NULL,
    link          VARCHAR(100) NOT NULL,
    city          VARCHAR(100),
    status        VARCHAR(100),
    b_avatar      VARCHAR(500),
    s_avatar      VARCHAR(500),
    email         VARCHAR(100),
    birthday      DATE NOT NULL,
    last_activity TIMESTAMP NOT NULL,

    UNIQUE(phone),
    UNIQUE(email)
);

-- профили пользователей -------
CREATE TABLE user_profiles (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    posts          INT NOT NULL,
    friends        INT NOT NULL,
    follows        INT NOT NULL,
    communities    INT NOT NULL,
    photos         INT NOT NULL,
    goods          INT NOT NULL,
    docs           INT NOT NULL,
    tracks         INT NOT NULL,
    videos         INT NOT NULL,
    articles       INT NOT NULL,
    planners       INT NOT NULL,
    avatar_id      INT,
    survey         INT NOT NULL,
    saved_playlist VARCHAR(100) NOT NULL
);
CREATE UNIQUE INDEX user_profiles_unq ON user_profiles (user_id, id);

-- местоположения пользователей -------
CREATE TABLE user_locations (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    city_ru    VARCHAR(100),
    city_en    VARCHAR(100),
    region_ru  VARCHAR(100),
    region_en  VARCHAR(100),
    country_ru VARCHAR(100),
    country_en VARCHAR(100)
);
CREATE INDEX user_locations_user_idx ON user_locations (user_id);

-- айпи пользователей -------
CREATE TABLE ip_users (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    ip      VARCHAR NOT NULL
);
CREATE INDEX ip_users_user_idx ON ip_users (user_id);

-- подписчики -------
CREATE TABLE follows (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    followed_user INT NOT NULL,
    view          BOOLEAN NOT NULL DEFAULT false,
    visited       INT NOT NULL
);
CREATE UNIQUE INDEX follows_user_followed_unq ON follows (user_id, followed_user);

-- друзья -------
CREATE TABLE friends (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    target_user_id INT NOT NULL,
    visited        INT NOT NULL
);
CREATE UNIQUE INDEX friends_user_target_unq ON friends (user_id, target_user_id);

-- Анкета аккаунта -------
CREATE TABLE user_anketas (
    id                    SERIAL PRIMARY KEY,
    user_id               INT NOT NULL,
    political_preferences VARCHAR(500),
    worldview             VARCHAR(500),
    mainthing_in_life     VARCHAR(500),
    mainthing_in_people   VARCHAR(500),
    attitude_to_smoking   VARCHAR(500),
    attitude_to_alcohol   VARCHAR(500),
    inspiration           VARCHAR(500)
);
CREATE UNIQUE INDEX user_anketas_unq ON user_anketas (user_id, id);

-- Причина удаления аккаунта -------
CREATE TABLE user_delete_anketas (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    answer  "char" NOT NULL,
    other   VARCHAR(200),
    created TIMESTAMP NOT NULL
);
CREATE INDEX user_delete_anketas_idx ON user_delete_anketas (user_id);

-- Статус отношений -------
CREATE TABLE user_love_statuss (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    male_status   VARCHAR(6),
    female_status VARCHAR(6)
);
CREATE UNIQUE INDEX user_love_statuss_unq ON user_love_statuss (user_id, id);


-- Муж/Жена -------
CREATE TABLE user_partner_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_partner_ones_unq ON user_partner_ones (user_id, target_id);

-- Мама -------
CREATE TABLE user_mom_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_mom_ones_unq ON user_mom_ones (user_id, target_id);

-- Папа -------
CREATE TABLE user_dad_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_dad_ones_unq ON user_dad_ones (user_id, target_id);

-- Братья, сёстры -------
CREATE TABLE user_brother_sisters (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_brother_sisters_ones_unq ON user_brother_sisters (user_id, target_id);

-- Дети -------
CREATE TABLE user_children_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_children_ones_unq ON user_children_ones (id, target_id);

-- Внуки -------
CREATE TABLE user_grandsons_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_grandsons_ones_unq ON user_grandsons_ones (user_id, target_id);

-- Коллеги -------
CREATE TABLE user_colleagues_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_colleagues_ones_unq ON user_colleagues_ones (user_id, target_id);

-- Черный список -------
CREATE TABLE user_blocks (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_blocks_unq ON user_blocks (user_id, target_id);

------------------
------------------
-- Список ключей новостей, уведомлений или рекомендаций (если пользователь хочет их группировать) -------
CREATE TABLE list_user_communities_keys (
    id    SERIAL PRIMARY KEY,
    types "char" NOT NULL,       -- тип списка: 0 - неактивен, 1 - основной, 2 - пользовательский
    name  VARCHAR(100) NOT NULL, -- название
    owner INT NOT NULL           -- владелец
);

-- Ключи рекомендаций -------
CREATE TABLE featured_user_communities (
    id            SERIAL PRIMARY KEY,
    owner         INT NOT NULL,                   -- кто получает рекомендации
    list_id       INT,                            -- список, если есть
    user_id       INT,                            -- рекомендуемый друг
    community_id  INT,                            -- рекомендуемое сообщество
    mute          BOOLEAN NOT NULL DEFAULT false, -- не получать рекомендации источника
    sleep         TIMESTAMP                       -- не получать рекомендации источника до указанного времени

);

-- Ключи новостей -------
CREATE TABLE news_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                   -- кто получает новости
    list_id      INT,
    user_id      INT,                            -- новости друга
    community_id INT,                            -- новости сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать новости источника
    sleep        TIMESTAMP                       -- не получать новости источника до указанного времени
);
-- Ключи уыедомлений -------
CREATE TABLE notify_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                   -- кто получает уведомления
    list_id      INT,
    user_id      INT,                            -- уведомления друга
    community_id INT,                            -- уведомления сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать уведомления источника
    sleep        TIMESTAMP                       -- не получать уведомления источника до указанного времени
);

-- Настройка дизайна -------
CREATE TABLE design_settings (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    background "char" NOT NULL,

    CONSTRAINT fk_design_settings
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);
CREATE UNIQUE INDEX design_settings_unq ON design_settings (user_id, id);

-- Настройки приватности пользователя -------
-- 1:Все пользователи; 4:Друзья; 5:Друзья и друзья друзей;6:Только я
-- 17:Друзья, кроме; 18:Некоторые друзья
CREATE TABLE user_privates (
    id                SERIAL PRIMARY KEY,
    user_id           INT NOT NULL,
    can_see_all       "char" NOT NULL, -- Для кого профиль открыт...
    can_see_community "char" NOT NULL, -- Кто видит сообщества
    can_see_info      "char" NOT NULL,      -- Кто видит информацию
    can_see_friend    "char" NOT NULL,    -- Кто видит друзей
    can_send_message  "char" NOT NULL,  -- Кто пишет сообщения
    can_add_in_chat   "char" NOT NULL,   -- Кто приглашает в беседы
    can_see_post      "char" NOT NULL,      -- Кто видит записи
    can_see_photo     "char" NOT NULL,     -- Кто видит фотографии
    can_see_good      "char" NOT NULL,      -- Кто видит товары
    can_see_video     "char" NOT NULL,     -- Кто видит видеозаписи
    can_see_music     "char" NOT NULL,     -- Кто видит аудиозапис
    can_see_planner   "char" NOT NULL,   -- Кто видит раздел планирования
    can_see_doc       "char" NOT NULL,       -- Кто видит документы
    can_see_survey    "char" NOT NULL
);
CREATE UNIQUE INDEX user_privates_unq ON user_privates (user_id, id);

-- Уведомления профиля -------
CREATE TABLE user_profile_notifications (
    id                   SERIAL PRIMARY KEY,
    user_id              INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    community_invite     BOOLEAN NOT NULL DEFAULT true
);
CREATE UNIQUE INDEX user_profile_notifications_unq ON user_profile_notifications (user_id, id);

------------------
------------------
-- Смайлы и стикеры

-- Популярные смайлы -------
CREATE TABLE user_populate_smiles (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    smile_id  INT NOT NULL,
    count     INT NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX user_populate_smiles_unq ON user_populate_smiles (user_id, smile_id);

-- Популярные стикеры -------
CREATE TABLE user_populate_stickers (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    sticker_id  INT NOT NULL,
    count       INT NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX user_populate_stickers_unq ON user_populate_stickers (user_id, sticker_id);
