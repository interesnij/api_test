-- Your SQL goes here

CREATE TABLE community_visible_perms (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL, 

    can_see_info             "char",
    can_see_community        "char",
    can_see_member           "char",
    can_send_message         "char",
    can_add_in_chat          "char",
    can_see_doc              "char",
    can_see_music            "char",
    can_see_survey           "char",
    can_see_post             "char",
    can_see_post_comment     "char",
    can_see_photo            "char",
    can_see_photo_comment    "char",
    can_see_good             "char",
    can_see_good_comment     "char",
    can_see_video            "char",
    can_see_video_comment    "char",
    can_see_planner          "char",
    can_see_planner_comment  "char",
    can_see_forum            "char",
    can_see_forum_comment    "char"
);
CREATE UNIQUE INDEX community_visible_perms_unq ON community_visible_perms (user_id, id);

CREATE TABLE community_work_perms (
    id                      SERIAL PRIMARY KEY,
    user_id                 INT NOT NULL,

    can_copy_post            "char",
    can_copy_photo           "char",
    can_copy_good            "char",
    can_copy_video           "char",
    can_copy_planner         "char",
    can_copy_doc             "char",
    can_copy_music           "char",
    can_copy_survey          "char",

    can_work_post          "char",
    can_work_photo         "char",
    can_work_good          "char",
    can_work_video         "char",
    can_work_planner       "char",
    can_work_doc           "char",
    can_work_music         "char",
    can_work_survey        "char"
);
CREATE UNIQUE INDEX community_work_perms_unq ON community_work_perms (user_id, id);
