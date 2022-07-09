use crate::schema;
use crate::schema::{
    user_post_list_collections,
    community_post_list_collections,
    post_list_perms,
    post_list_reposts,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
};
use diesel::prelude::*;
use actix_web::web::Json;
use crate::models::PostComment;


/////// UserPostListCollection //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct UserPostListCollection {
    pub id:           i32,
    pub user_id:      i32,
    pub post_list_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_post_list_collections"]
pub struct NewUserPostListCollection {
    pub user_id:      i32,
    pub post_list_id: i32,
}

/////// CommunityPostListCollection //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPostListCollection {
    pub id:           i32,
    pub community_id: i32,
    pub post_list_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_post_list_collections"]
pub struct NewCommunityPostListCollection {
    pub community_id: i32,
    pub post_list_id: i32,
}

/////// PostListPerm //////
// 'a' Активно
// 'b' Не активно
// 'c' Нет значения
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostListPerm {
    pub id:              i32,
    pub user_id:         i32,
    pub post_list_id:    i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_list_perms"]
pub struct NewPostListPerm {
    pub user_id:         i32,
    pub post_list_id:    i32,
    pub can_see_item:    Option<String>,
    pub can_see_comment: Option<String>,
    pub create_item:     Option<String>,
    pub create_comment:  Option<String>,
    pub can_copy:        Option<String>,
}


/////// PostVote//////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostVote {
    pub id:       i32,
    pub vote:     i16,
    pub user_id:  i32,
    pub post_id:  i32,
    pub reaction: i16,
}

#[derive(Deserialize, Insertable)]
#[table_name="post_votes"]
pub struct NewPostVote {
    pub vote:     i16,
    pub user_id:  i32,
    pub post_id:  i32,
    pub reaction: i16,
}
/////// PostCommentVote //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostCommentVote {
    pub id:              i32,
    pub vote:            i16,
    pub user_id:         i32,
    pub post_comment_id: i32,
    pub reaction:        i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comment_votes"]
pub struct NewPostCommentVote {
    pub vote:            i16,
    pub user_id:         i32,
    pub post_comment_id: i32,
    pub reaction:        i16,
}

/////// PostListRepost //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostListRepost {
    pub id:           i32,
    pub post_list_id: i32,
    pub post_id:      Option<i32>,
    pub message_id:   Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_list_reposts"]
pub struct NewPostListRepost {
    pub post_list_id: i32,
    pub post_id:      Option<i32>,
    pub message_id:   Option<i32>,
}

/////// PostReaction //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostReaction {
    pub id:       i32,
    pub post_id:  i32,
    pub field_1:  i32,
    pub field_2:  i32,
    pub field_3:  i32,
    pub field_4:  i32,
    pub field_5:  i32,
    pub field_6:  i32,
    pub field_7:  i32,
    pub field_8:  i32,
    pub field_9:  i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
}

impl PostReaction {
    pub fn count_reactions_of_types(&self, types: i16) -> i32 {
        let format_types: i32 = types.into();
        let count = match format_types {
            1 => self.field_1,
            2 => self.field_2,
            3 => self.field_3,
            4 => self.field_4,
            5 => self.field_5,
            6 => self.field_6,
            7 => self.field_7,
            8 => self.field_8,
            9 => self.field_9,
            10 => self.field_10,
            11 => self.field_11,
            12 => self.field_12,
            13 => self.field_13,
            14 => self.field_14,
            15 => self.field_15,
            16 => self.field_16,
            _ => 0,
        };
        return count;
    }
    pub fn count_reactions_of_types_ru(&self, types: i16) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_reactions_of_types(types),
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn update_model(
        &self,
        new_types: i16,
        old_types_option: Option<i16>,
        plus: bool,
    ) -> &PostReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::post_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::post_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::post_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::post_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::post_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::post_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::post_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::post_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::post_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::post_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::post_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::post_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::post_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::post_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::post_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::post_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::post_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::post_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::post_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::post_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::post_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::post_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::post_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::post_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::post_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::post_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::post_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::post_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::post_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::post_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::post_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::post_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::post_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::post_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<PostReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::post_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::post_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::post_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::post_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::post_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::post_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::post_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::post_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::post_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::post_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::post_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::post_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::post_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::post_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::post_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::post_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::post_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::post_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::post_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::post_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::post_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::post_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::post_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::post_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::post_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::post_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::post_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::post_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::post_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::post_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::post_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::post_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::post_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::post_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<PostReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="post_reactions"]
pub struct NewPostReaction {
    pub post_id:     i32,
    pub field_1:  i32,
    pub field_2:  i32,
    pub field_3:  i32,
    pub field_4:  i32,
    pub field_5:  i32,
    pub field_6:  i32,
    pub field_7:  i32,
    pub field_8:  i32,
    pub field_9:  i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
}


/////// PostCommentReaction //////
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(PostComment)]
pub struct PostCommentReaction {
    pub id:              i32,
    pub post_comment_id: i32,
    pub field_1:  i32,
    pub field_2:  i32,
    pub field_3:  i32,
    pub field_4:  i32,
    pub field_5:  i32,
    pub field_6:  i32,
    pub field_7:  i32,
    pub field_8:  i32,
    pub field_9:  i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
}
impl PostCommentReaction {
    pub fn count_reactions_of_types(&self, types: i16) -> i32 {
        let format_types: i32 = types.into();
        let count = match format_types {
            1 => self.field_1,
            2 => self.field_2,
            3 => self.field_3,
            4 => self.field_4,
            5 => self.field_5,
            6 => self.field_6,
            7 => self.field_7,
            8 => self.field_8,
            9 => self.field_9,
            10 => self.field_10,
            11 => self.field_11,
            12 => self.field_12,
            13 => self.field_13,
            14 => self.field_14,
            15 => self.field_15,
            16 => self.field_16,
            _ => 0,
        };
        return count;
    }
    pub fn count_reactions_of_types_ru(&self, types: i16) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count_reactions_of_types(types),
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }
    pub fn update_model(
        &self,
        new_types: i16,
        old_types_option: Option<i16>,
        plus: bool,
    ) -> &PostCommentReaction {
        let _connection = establish_connection();
        if old_types_option.is_some() {
            let old_types = old_types_option.unwrap();
            match new_types {
                1 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_1.eq(self.field_1 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::post_comment_reactions::field_2.eq(self.field_2 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_3.eq(self.field_3 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_4.eq(self.field_4 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_5.eq(self.field_5 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_6.eq(self.field_6 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_7.eq(self.field_7 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_8.eq(self.field_8 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_9.eq(self.field_9 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_10.eq(self.field_10 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_11.eq(self.field_11 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_12.eq(self.field_12 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_13.eq(self.field_13 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_14.eq(self.field_14 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_15.eq(self.field_15 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::post_comment_reactions::field_16.eq(self.field_16 + 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
            };

            match old_types {
                1 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_1.eq(self.field_1 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                2 => diesel::update(self).
                    set(schema::post_comment_reactions::field_2.eq(self.field_2 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                3 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_3.eq(self.field_3 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                4 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_4.eq(self.field_4 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                5 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_5.eq(self.field_5 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                6 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_6.eq(self.field_6 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                7 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_7.eq(self.field_7 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                8 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_8.eq(self.field_8 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                9 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_9.eq(self.field_9 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                10 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_10.eq(self.field_10 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                11 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_11.eq(self.field_11 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                12 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_12.eq(self.field_12 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                13 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_13.eq(self.field_13 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                14 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_14.eq(self.field_14 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                15 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_15.eq(self.field_15 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                16 => diesel::update(self)
                    .set(schema::post_comment_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
                _ => diesel::update(self)
                    .set(schema::post_comment_reactions::field_16.eq(self.field_16 - 1))
                    .get_result::<PostCommentReaction>(&_connection)
                    .expect("Error."),
            };
            return &self;
        }
        else {
            if plus {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_1.eq(self.field_1 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::post_comment_reactions::field_2.eq(self.field_2 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_3.eq(self.field_3 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_4.eq(self.field_4 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_5.eq(self.field_5 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_6.eq(self.field_6 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_7.eq(self.field_7 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_8.eq(self.field_8 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_9.eq(self.field_9 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_10.eq(self.field_10 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_11.eq(self.field_11 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_12.eq(self.field_12 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_13.eq(self.field_13 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_14.eq(self.field_14 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_15.eq(self.field_15 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::post_comment_reactions::field_16.eq(self.field_16 + 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                };
            }
            else {
                match new_types {
                    1 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_1.eq(self.field_1 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    2 => diesel::update(self).
                        set(schema::post_comment_reactions::field_2.eq(self.field_2 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    3 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_3.eq(self.field_3 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    4 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_4.eq(self.field_4 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    5 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_5.eq(self.field_5 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    6 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_6.eq(self.field_6 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    7 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_7.eq(self.field_7 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    8 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_8.eq(self.field_8 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    9 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_9.eq(self.field_9 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    10 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_10.eq(self.field_10 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    11 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_11.eq(self.field_11 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    12 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_12.eq(self.field_12 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    13 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_13.eq(self.field_13 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    14 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_14.eq(self.field_14 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    15 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_15.eq(self.field_15 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    16 => diesel::update(self)
                        .set(schema::post_comment_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                    _ => diesel::update(self)
                        .set(schema::post_comment_reactions::field_16.eq(self.field_16 - 1))
                        .get_result::<PostCommentReaction>(&_connection)
                        .expect("Error."),
                };
            }
            return &self;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="post_comment_reactions"]
pub struct NewPostCommentReaction {
    pub post_comment_id: i32,
    pub field_1:  i32,
    pub field_2:  i32,
    pub field_3:  i32,
    pub field_4:  i32,
    pub field_5:  i32,
    pub field_6:  i32,
    pub field_7:  i32,
    pub field_8:  i32,
    pub field_9:  i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
}
