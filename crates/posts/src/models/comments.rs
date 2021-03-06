use crate::schema;
use diesel::prelude::*;

use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    JsonItemReactions,
    //CommentsJson,
    //RepliesJson,
    CardCommentJson,
    CardReplyJson,
    //ReactionsCommentJson,
    ReactionBlockJson,
    RepliesSmallJson,
};
use actix_web::web::Json;
use crate::models::{Post, PostCommentReaction, PostList};
use crate::schema::post_comments;


/////// PostComment //////

// 'a' Опубликованный
// 'b' Изменённый
// 'c' Удаленый
// 'd' Изменённый Удаленый
// 'e' Закрытый модератором
// 'f' Закрытый Удаленый

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct PostComment {
    pub id:          i32,
    pub post_id:     i32,
    pub user_id:     i32,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub sticker_id:  Option<i32>,
    pub parent_id:   Option<i32>,
    pub content:     Option<String>,
    pub attach:      Option<String>,
    pub types:       String,
    pub created:     chrono::NaiveDateTime,
    pub repost:      i32,
    pub reactions:   i32,
    pub replies:     i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="post_comments"]
pub struct NewPostComment {
    pub post_id:     i32,
    pub user_id:     i32,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub sticker_id:  Option<i32>,
    pub parent_id:   Option<i32>,
    pub content:     Option<String>,
    pub attach:      Option<String>,
    pub types:       String,
    pub created:     chrono::NaiveDateTime,
    pub repost:      i32,
    pub reactions:   i32,
    pub replies:     i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="post_comments"]
pub struct EditPostComment {
    pub content:    Option<String>,
    pub attach:     Option<String>,
}

impl PostComment {
    pub fn get_6_reactions_of_types (
        &self, types: &i16, user_reaction: Option<i16>, count: i32
    ) -> ReactionBlockJson {
        use crate::schema::post_comment_votes::dsl::post_comment_votes;
        use crate::utils::CardReactionPostJson;
        use crate::models::PostCommentVote;

        let _connection = establish_connection();
        let votes = post_comment_votes
            .filter(schema::post_comment_votes::post_comment_id.eq(self.id))
            .filter(schema::post_comment_votes::reaction.eq(types))
            .limit(6)
            .load::<PostCommentVote>(&_connection)
            .expect("E");

        let mut user_json = Vec::new();
        for _item in votes.iter() {
            user_json.push (
                CardReactionPostJson {
                    owner_name:        _item.owner_name.clone(),
                    owner_link:        _item.owner_name.clone(),
                    owner_image:       _item.owner_image.clone(),
                    is_user_reaction: &user_reaction.unwrap() == types,
                }
            );
        }
        return ReactionBlockJson {
                status:   200,
                count:    count,
                reaction: *types,
                users:    user_json,
            };
    }
    pub fn get_reactions_of_types (
        &self, types: &i16, user_reaction: Option<i16>, count: i32
    ) -> ReactionBlockJson {
        use crate::schema::post_comment_votes::dsl::post_comment_votes;
        use crate::utils::CardReactionPostJson;
        use crate::models::PostCommentVote;

        let _connection = establish_connection();
        let votes = post_comment_votes
            .filter(schema::post_comment_votes::post_comment_id.eq(self.id))
            .filter(schema::post_comment_votes::reaction.eq(types))
            .load::<PostCommentVote>(&_connection)
            .expect("E");

        let mut user_json = Vec::new();
        for _item in votes.iter() {
            user_json.push (
                CardReactionPostJson {
                    owner_name:  _item.owner_name.clone(),
                    owner_link:  _item.owner_name.clone(),
                    owner_image: _item.owner_image.clone(),
                    is_user_reaction: &user_reaction.unwrap() == types,
                }
            );
        }
        return ReactionBlockJson {
                status:   200,
                count:    count,
                reaction: *types,
                users:    user_json,
            };
    }

    pub fn get_reactions_json (&self, user_id: i32, reactions_list: Vec<i16>) -> Option<Vec<ReactionBlockJson>> {
        // получаем реакции и отреагировавших
        let reactions_blocks: Option<Vec<ReactionBlockJson>>;
        if reactions_list.len() == 0 {
            reactions_blocks = None;
        }
        else {
            let mut reactions_json: Vec<ReactionBlockJson> = Vec::new();
            let object_reactions_count = self.get_or_create_react_model();
            let mut user_reaction = 0;

            if self.is_have_user_reaction(user_id) {
                user_reaction = self.get_user_reaction(user_id);
            }

            for reaction in reactions_list.iter() {
                let count = object_reactions_count.count_reactions_of_types(*reaction);
                if count > 0 {
                    reactions_json.push(self.get_6_reactions_of_types(reaction, Some(user_reaction), count));
                }
            }
            reactions_blocks = Some(reactions_json);
        }
        return reactions_blocks;
    }

    pub fn get_comment_json (&self, user_id: i32, reactions_list: Vec<i16>) -> CardCommentJson {
        let card = CardCommentJson {
            content:        self.content.clone(),
            owner_name:     self.owner_name.clone(),
            owner_link:     self.owner_link.clone(),
            owner_image:    self.owner_image.clone(),
            created:        self.created.format("%d-%m-%Y в %H:%M").to_string(),
            reactions:      self.reactions,
            types:          self.get_code(),
            replies:        self.replies,
            reactions_list: self.get_reactions_json(user_id, reactions_list.clone()),
            items:          None,
        };
        return card;
    }
    pub fn get_reply_json (&self, user_id: i32, reactions_list: Vec<i16>) -> CardReplyJson {
        let card = CardReplyJson {
            content:        self.content.clone(),
            owner_name:     self.owner_name.clone(),
            owner_link:     self.owner_link.clone(),
            owner_image:    self.owner_image.clone(),
            created:        self.created.format("%d-%m-%Y в %H:%M").to_string(),
            reactions:      self.reactions,
            types:          self.get_code(),
            reactions_list: self.get_reactions_json(user_id, reactions_list.clone()),
            items:          None,
        };
        return card;
    }
    pub fn get_replies_json (
        &self,
        user_id: i32,
        reactions_list: Vec<i16>,
        page: i32, limit: i32
    ) -> RepliesSmallJson {
        let mut comments_json = Vec::new();
        let mut next_page_number = 0;
        let have_next: i32;

        if page > 1 {
            have_next = page * limit + 1;
            for c in self.get_replies(limit.into(), have_next.into()).iter() {
                let r_list = reactions_list.clone();
                comments_json.push(c.get_reply_json(user_id, r_list));
            }
        }
        else {
            have_next = limit + 1;
            for c in self.get_replies(limit.into(), 0).iter() {
                let r_list = reactions_list.clone();
                comments_json.push(c.get_reply_json(user_id, r_list));
            }
        }
        if self.get_replies(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return RepliesSmallJson {
            replies:   comments_json,
            next_page: next_page_number,
        };
    }
    pub fn get_replies(&self, limit: i64, offset: i64) -> Vec<PostComment> {
        use crate::schema::post_comments::dsl::post_comments;

        let _connection = establish_connection();
        return post_comments
            .filter(schema::post_comments::parent_id.eq(self.id))
            .filter(schema::post_comments::types.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<PostComment>(&_connection)
            .expect("E.");
    }
    pub fn is_deleted(&self) -> bool {
        return self.types == "c" && self.types == "d";
    }
    pub fn is_closed(&self) -> bool {
        return self.types == "e" && self.types == "f";
    }

    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post_comment(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "cpo".to_string() + &self.get_str_id();
    }

    pub fn get_item(&self) -> Post {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::id.eq(self.post_id))
            .filter(schema::posts::types.eq("a"))
            .load::<Post>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_list(&self) -> PostList {
        return self.get_item().get_list();
    }
    pub fn get_parent(&self) -> PostComment {
        use crate::schema::post_comments::dsl::post_comments;

        let _connection = establish_connection();
        return post_comments
            .filter(schema::post_comments::id.eq(self.parent_id.unwrap()))
            .filter(schema::post_comments::types.eq_any(vec!["a", "b"]))
            .load::<PostComment>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_manager_type(&self) -> i16 {
        if self.parent_id.is_some() {
            return 87;
        }
        else {
            return 81;
        }
    }

    pub fn count_replies(&self) -> i32 {
        return self.replies;
    }
    pub fn count_replies_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.replies,
            " ответ".to_string(),
            " ответа".to_string(),
            " ответов".to_string(),
        );
    }
    pub fn close_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "e".to_string(),
            "b" => "f".to_string(),
            _ => "e".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment - 1))
            .get_result::<Post>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::post_comments::types.eq(close_case))
            .get_result::<PostComment>(&_connection)
            .expect("E");

        //hide_wall_notify_items(self.get_manager_type(), self.id);
    }
    pub fn unclose_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "e" => "a".to_string(),
            "f" => "b".to_string(),
            _ => "a".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment + 1))
            .get_result::<Post>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::post_comments::types.eq(close_case))
            .get_result::<PostComment>(&_connection)
            .expect("E");

        //show_wall_notify_items(self.get_manager_type(), self.id);
    }

    pub fn delete_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "c".to_string(),
            "b" => "d".to_string(),
            _ => "c".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment - 1))
            .get_result::<Post>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::post_comments::types.eq(close_case))
            .get_result::<PostComment>(&_connection)
            .expect("E");

        //hide_wall_notify_items(self.get_manager_type(), self.id);
    }
    pub fn restore_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "c" => "a".to_string(),
            "d" => "b".to_string(),
            _ => "a".to_string(),
        };
        let item = self.get_item();
        diesel::update(&item)
            .set(schema::posts::comment.eq(item.comment + 1))
            .get_result::<Post>(&_connection)
            .expect("E");

        diesel::update(self)
            .set(schema::post_comments::types.eq(close_case))
            .get_result::<PostComment>(&_connection)
            .expect("E");

        //show_wall_notify_items(self.get_manager_type(), self.id);
    }
    pub fn get_count_attach(&self) -> String {
        if self.attach.is_some() {
            let length = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>().len();
            if length == 1 {
                return "files_one".to_string();
            }
            else if length == 2 {
                return "files_two".to_string();
            }
        }
        return "files_null".to_string();
    }
    pub fn get_format_text(&self) -> String {
        if self.content.is_some() {
            let unwrap = self.content.as_ref().unwrap();
            if unwrap.len() <= 101 {
                return self.content.as_ref().unwrap().to_string();
            }
            else {
                let new_str = unwrap[..100].to_owned() + &"<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>" + &unwrap[101..] + &"</span>";
                return new_str;
            }
        } else { return "".to_string(); }
    }

    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
    }

    pub fn get_or_create_react_model(&self) -> PostCommentReaction {
        use crate::schema::post_comment_reactions::dsl::post_comment_reactions;
        use crate::models::NewPostCommentReaction;

        let _connection = establish_connection();
        let _react_model = post_comment_reactions
            .filter(schema::post_comment_reactions::post_comment_id.eq(self.id))
            .load::<PostCommentReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewPostCommentReaction {
                post_comment_id: self.id,
                field_1:  0,
                field_2:  0,
                field_3:  0,
                field_4:  0,
                field_5:  0,
                field_6:  0,
                field_7:  0,
                field_8:  0,
                field_9:  0,
                field_10: 0,
                field_11: 0,
                field_12: 0,
                field_13: 0,
                field_14: 0,
                field_15: 0,
                field_16: 0,
            };
            let _react_model = diesel::insert_into(schema::post_comment_reactions::table)
                .values(&new_react_model)
                .get_result::<PostCommentReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction (
        &self,
        user_id: i32,
        types: i16,
        owner_name: String,
        owner_link: String,
        owner_image: Option<String>
    ) -> Json<JsonItemReactions> {
        use crate::schema::post_comment_votes::dsl::post_comment_votes;
        use crate::models::{NewPostCommentVote, PostCommentVote};

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.is_user_can_see_el(user_id) && list.is_user_can_see_comment(user_id) {

            let votes = post_comment_votes
                .filter(schema::post_comment_votes::user_id.eq(user_id))
                .filter(schema::post_comment_votes::post_comment_id.eq(self.id))
                .load::<PostCommentVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(post_comment_votes
                        .filter(schema::post_comment_votes::user_id.eq(user_id))
                        .filter(schema::post_comment_votes::post_comment_id.eq(self.id))
                        )
                        .execute(&_connection)
                        .expect("E");
                    react_model.update_model(types, None, false);
                    self.minus_reactions(1);
                }
                // если пользователь уже реагировал другой реакцией на этот товар
                else {
                    old_type = vote.reaction;
                    diesel::update(&vote)
                        .set(schema::post_comment_votes::reaction.eq(types))
                        .get_result::<PostCommentVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewPostCommentVote {
                    vote:            1,
                    user_id:         user_id,
                    post_comment_id: self.id,
                    reaction:        types,
                    owner_name:      owner_name,
                    owner_link:      owner_link,
                    owner_image:     owner_image,
                };
                diesel::insert_into(schema::post_comment_votes::table)
                    .values(&new_vote)
                    .get_result::<PostCommentVote>(&_connection)
                    .expect("Error.");

                react_model.update_model(types, None, true);
                self.plus_reactions(1, user_id);
                new_plus = true;
            }
        }

        let mut data: Vec<i32> = Vec::new();
        data.push(self.reactions);
        data.push(react_model.field_1);
        data.push(react_model.field_2);
        data.push(react_model.field_3);
        data.push(react_model.field_4);
        data.push(react_model.field_5);
        data.push(react_model.field_6);
        data.push(react_model.field_7);
        data.push(react_model.field_8);
        data.push(react_model.field_9);
        data.push(react_model.field_10);
        data.push(react_model.field_11);
        data.push(react_model.field_12);
        data.push(react_model.field_13);
        data.push(react_model.field_14);
        data.push(react_model.field_15);
        data.push(react_model.field_16);

        let types_usize: usize = types as usize;
        if old_type != 0 {
            let old_type_usize: usize = old_type as usize;
            data[types_usize] = data[types_usize] + 1;
            data[old_type_usize] = data[old_type_usize] - 1;
        }
        else if new_plus {
            data[types_usize] = data[types_usize] + 1;
            data[0] = data[0] + 1;
        }
        else {
            data[types_usize] = data[types_usize] - 1;
            data[0] = data[0] - 1;
        }

        return Json(JsonItemReactions {data});
    }

    pub fn count_reactions_of_types(&self, types: i16) -> i32 {
        let react_model = self.get_or_create_react_model();
        let format_types: i32 = types.into();
        let count = match format_types {
            1 => react_model.field_1,
            2 => react_model.field_2,
            3 => react_model.field_3,
            4 => react_model.field_4,
            5 => react_model.field_5,
            6 => react_model.field_6,
            7 => react_model.field_7,
            8 => react_model.field_8,
            9 => react_model.field_9,
            10 => react_model.field_10,
            11 => react_model.field_11,
            12 => react_model.field_12,
            13 => react_model.field_13,
            14 => react_model.field_14,
            15 => react_model.field_15,
            16 => react_model.field_16,
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

    pub fn count_reactions_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.reactions,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn is_have_reactions(&self) -> bool {
        return self.reactions > 0;
    }

    pub fn reactions_ids(&self) -> Vec<i32> {
        use crate::schema::post_comment_votes::dsl::post_comment_votes;

        let _connection = establish_connection();
        let votes = post_comment_votes
            .filter(schema::post_comment_votes::post_comment_id.eq(self.id))
            .select(schema::post_comment_votes::user_id)
            .load::<i32>(&_connection)
            .expect("E");
        return votes;
    }

    pub fn is_have_user_reaction(&self, user_id: i32) -> bool {
        return self.reactions_ids().iter().any(|&i| i==user_id);
    }

    pub fn get_user_reaction(&self, user_id: i32) -> i16 {
        use crate::schema::post_comment_votes::dsl::post_comment_votes;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"
        let _connection = establish_connection();
        let vote = post_comment_votes
            .filter(schema::post_comment_votes::user_id.eq(user_id))
            .filter(schema::post_comment_votes::post_comment_id.eq(self.id))
            .select(schema::post_comment_votes::reaction)
            .load::<i16>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote;
    }

    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::post_comments::reactions.eq(self.reactions + count))
            .get_result::<PostComment>(&_connection)
            .expect("Error.");
    }
    pub fn minus_reactions(&self, count: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::post_comments::reactions.eq(self.reactions - count))
            .get_result::<PostComment>(&_connection)
            .expect("Error.");
    }
    pub fn get_small_content(&self) -> String {
        if self.content.is_some() {
            let _content = self.content.as_deref().unwrap();
            if _content.len() > 50 {
                return _content[..50].to_string();
            }
            else {
                return _content.to_string();
            }
        }
        else {
            return "".to_string();
        }
    }
}
