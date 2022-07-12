use crate::schema;
use diesel::prelude::*;

use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    JsonPosition,
    JsonItemReactions,
    //PostsJson,
    CardParentPostJson,
    RepostsPostJson,
    CardPostJson,
    ReactionBlockJson,
    PostDetailJson,
    CardRepostPostJson,
    CommentsSmallJson,
};
use actix_web::web::Json;
use crate::models::{
    PostComment, NewPostComment,
    PostReaction, NewPostReaction,
    PostList, PostRepost,
};
use crate::schema::posts;

/////// Post //////

//////////// тип
    // 'a' Опубликовано
    // 'b' Закрепленый
    // 'c' Удаленый
    // 'd' Черновик владельца
    // 'e' Черновик предложки
    // 'f' Предложка сообщества
    // 'g' Предложка пользователя
    // 'h' Закрыто модератором
    // 'i' Удаленый предложенный в сообщество
    // 'y' Удаленый предложенный у пользователя
    // 'c' Удаленый


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Post {
    pub id:              i32,
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub owner_name:      String,
    pub owner_link:      String,
    pub owner_image:     Option<String>,
    pub post_list_id:    i32,
    pub types:           String,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub created:         chrono::NaiveDateTime,
    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub is_signature:    bool,
    pub parent_id:       Option<i32>,
    pub reactions:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub content:         Option<String>,
    pub community_id:    Option<i32>,
    pub user_id:         i32,
    pub owner_name:      String,
    pub owner_link:      String,
    pub owner_image:     Option<String>,
    pub post_list_id:    i32,
    pub types:           String,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub created:         chrono::NaiveDateTime,
    pub comment:         i32,
    pub view:            i32,
    pub repost:          i32,
    pub copy:            i32,
    pub position:        i16,
    pub is_signature:    bool,
    pub parent_id:       Option<i32>,
    pub reactions:       i32,
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPost {
    pub content:         Option<String>,
    pub attach:          Option<String>,
    pub comment_enabled: bool,
    pub is_signature:    bool,
}
#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPostPosition {
    pub position: i16,
}

impl Post {
    pub fn get_comments_post_json (
        &self,
        user_id: i32,
        reactions_list: Vec<i16>,
        page: i32
    ) -> CommentsSmallJson {
        let mut comments_json = Vec::new();
        let mut next_page_number = 0;
        let comments: Vec<PostComment> - Vec::new();
        let count = self.comment;
        if page > 1 {
            let step = (page - 1) * 20;
            comments = self.get_comments(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            comments = self.get_comments(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        for c in comments.iter() {
            let r_list = reactions_list.clone();
            comments_json.push(c.get_comment_json(user_id, r_list));
        }

        return CommentsSmallJson {
            comments:  comments_json,
            next_page: next_page_number,
        };
    }

    pub fn get_parent_post_json (&self) -> Option<CardParentPostJson> {
        // получаем родительский пост
        let parent: Option<CardParentPostJson>;
        if self.parent_id.is_some() {
            let _parent = self.get_parent();
            parent = Some(CardParentPostJson {
                id:              _parent.id,
                content:         _parent.content.clone(),
                owner_name:      _parent.owner_name.clone(),
                owner_link:      _parent.owner_link.clone(),
                owner_image:     _parent.owner_image.clone(),
                attach:          _parent.attach.clone(),
                created:         _parent.created.format("%d-%m-%Y в %H:%M").to_string(),
            })
        }
        else {
            parent = None;
        }
        return parent;
    }
    pub fn get_6_reposts_post_json (&self) -> Option<RepostsPostJson> {
        // получаем репосты записи, если есть
        let reposts_window: Option<RepostsPostJson>;
        if self.repost > 0 {
            let mut reposts_json = Vec::new();
            for r in self.window_reposts().iter() {
                reposts_json.push (
                    CardRepostPostJson {
                        owner_name:  r.owner_name.clone(),
                        owner_link:  r.owner_name.clone(),
                        owner_image: r.owner_image.clone(),
                    }
                );
            }

            reposts_window = Some(RepostsPostJson {
                status:          200,
                message_reposts: self.message_reposts_count(),
                copy_count:      self.count_copy(),
                posts:           reposts_json,
            });
        }
        else {
            reposts_window = None;
        }
        return reposts_window;
    }
    pub fn get_reposts_post_json (&self, limit: i64, offset: i64) -> Option<RepostsPostJson> {
        // получаем репосты записи, если есть
        let reposts_window: Option<RepostsPostJson>;
        if self.repost > 0 {
            let mut reposts_json = Vec::new();
            for r in self.reposts(limit, offset).iter() {
                reposts_json.push (
                    CardRepostPostJson {
                        owner_name:  r.owner_name.clone(),
                        owner_link:  r.owner_name.clone(),
                        owner_image: r.owner_image.clone(),
                    }
                );
            }

            reposts_window = Some(RepostsPostJson {
                status:          200,
                message_reposts: self.message_reposts_count(),
                copy_count:      self.count_copy(),
                posts:           reposts_json,
            });
        }
        else {
            reposts_window = None;
        }
        return reposts_window;
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

    pub fn get_detail_post_json (&self, user_id: i32, page: i32) -> PostDetailJson {
        let list = self.get_list();
        let reactions_list = list.get_reactions_list();
        let mut prev: Option<i32> = None;
        let mut next: Option<i32> = None;
        let _posts = list.get_items();
        for (i, item) in _posts.iter().enumerate().rev() {
            if item.position == self.position {
                if (i + 1) != _posts.len() {
                    prev = Some(_posts[i + 1].id);
                };
                if i != 0 {
                    next = Some(_posts[i - 1].id);
                };
                break;
            }
        };
        return PostDetailJson {
                content:         self.content.clone(),
                owner_name:      self.owner_name.clone(),
                owner_link:      self.owner_link.clone(),
                owner_image:     self.owner_image.clone(),
                attach:          self.attach.clone(),
                comment_enabled: self.comment_enabled,
                created:         self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:         self.comment,
                view:            self.view,
                repost:          self.repost,
                is_signature:    self.is_signature,
                reactions:       self.reactions,
                types:           self.get_code(),
                parent:          self.get_parent_post_json(),
                reposts:         self.get_6_reposts_post_json(),
                reactions_list:  self.get_reactions_json(user_id, reactions_list.clone()),
                prev:            prev,
                next:            next,
                is_user_can_see_comments: list.is_user_can_see_comment(user_id),
                is_user_can_create_el: list.is_user_can_create_el(user_id),
                comments: self.get_comments_post_json(user_id, reactions_list.clone(), page),
            };
    }
    pub fn get_post_json (&self, user_id: i32, reactions_list: Vec<i16>,) -> CardPostJson {
        return CardPostJson {
                id:              self.id,
                content:         self.content.clone(),
                owner_name:      self.owner_name.clone(),
                owner_link:      self.owner_link.clone(),
                owner_image:     self.owner_image.clone(),
                attach:          self.attach.clone(),
                comment_enabled: self.comment_enabled,
                created:         self.created.format("%d-%m-%Y в %H:%M").to_string(),
                comment:         self.comment,
                view:            self.view,
                repost:          self.repost,
                is_signature:    self.is_signature,
                reactions:       self.reactions,
                types:           self.get_code(),
                parent:          self.get_parent_post_json(),
                reposts:         self.get_6_reposts_post_json(),
                reactions_list:  self.get_reactions_json(user_id, reactions_list),
            };
    }

    pub fn get_6_reactions_of_types (&self, types: &i16, user_reaction: Option<i16>, count: i32) -> ReactionBlockJson {
        use crate::schema::post_votes::dsl::post_votes;
        use crate::utils::CardReactionPostJson;
        use crate::models::PostVote;

        let _connection = establish_connection();
        let votes = post_votes
            .filter(schema::post_votes::post_id.eq(self.id))
            .filter(schema::post_votes::reaction.eq(types))
            .limit(6)
            .load::<PostVote>(&_connection)
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
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_post(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "pos".to_string() + &self.get_str_id();
    }
    pub fn get_folder(&self) -> String {
        return "posts".to_string();
    }

    pub fn message_reposts_count(&self) -> String {
        use crate::schema::post_reposts::dsl::post_reposts;

        let _connection = establish_connection();

        let count = post_reposts
            .filter(schema::post_reposts::post_id.eq(self.id))
            .load::<PostRepost>(&_connection)
            .expect("E.")
            .len();

        if count == 0 {
            return "".to_string();
        }
        else {
            return ", из них в сообщениях - ".to_string() + &count.to_string();
        }
    }

    pub fn get_or_create_react_model(&self) -> PostReaction {
        use crate::schema::post_reactions::dsl::post_reactions;

        let _connection = establish_connection();
        let _react_model = post_reactions
            .filter(schema::post_reactions::post_id.eq(self.id))
            .load::<PostReaction>(&_connection)
            .expect("E.");
        if _react_model.len() > 0 {
            return _react_model.into_iter().nth(0).unwrap();
        }
        else {
            let new_react_model = NewPostReaction {
                post_id:  self.id,
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
            let _react_model = diesel::insert_into(schema::post_reactions::table)
                .values(&new_react_model)
                .get_result::<PostReaction>(&_connection)
                .expect("Error.");

            return _react_model;
        }
    }

    pub fn send_reaction(
        &self,
        user_id: i32,
        types: i16,
        owner_name: String,
        owner_link: String,
        owner_image: Option<String>
    ) -> Json<JsonItemReactions> {
        use crate::schema::post_votes::dsl::post_votes;
        use crate::models::{PostVote, NewPostVote};

        let _connection = establish_connection();
        let list = self.get_list();
        let reactions_of_list = list.get_reactions_list();
        let react_model = self.get_or_create_react_model();
        let mut new_plus = false;
        let mut old_type = 0;

        if reactions_of_list.iter().any(|&i| i==types) && list.is_user_can_see_el(user_id) {
            let votes = post_votes
                .filter(schema::post_votes::user_id.eq(user_id))
                .filter(schema::post_votes::post_id.eq(self.id))
                .load::<PostVote>(&_connection)
                .expect("E.");

            // если пользователь уже реагировал на товар
            if votes.len() > 0 {
                let vote = votes.into_iter().nth(0).unwrap();

                // если пользователь уже реагировал этой реакцией на этот товар
                if vote.reaction == types {
                    diesel::delete(post_votes
                        .filter(schema::post_votes::user_id.eq(user_id))
                        .filter(schema::post_votes::post_id.eq(self.id))
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
                        .set(schema::post_votes::reaction.eq(types))
                        .get_result::<PostVote>(&_connection)
                        .expect("Error.");

                    react_model.update_model(types, Some(old_type), false);
                }
            }

            // если пользователь не реагировал на этот товар
            else {
                let new_vote = NewPostVote {
                    vote: 1,
                    user_id: user_id,
                    post_id: self.id,
                    reaction: types,
                    owner_name: owner_name,
                    owner_link: owner_link,
                    owner_image: owner_image,
                };
                diesel::insert_into(schema::post_votes::table)
                    .values(&new_vote)
                    .get_result::<PostVote>(&_connection)
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

    pub fn get_list(&self) -> PostList {
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        return post_lists
            .filter(schema::post_lists::id.eq(self.post_list_id))
            .filter(schema::post_lists::types.lt(10))
            .load::<PostList>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_playlist_image(&self) -> String {
        return "/static/images/news_small3.jpg".to_string();
    }

    pub fn create_parent_post (
        community_id: Option<i32>,
        user_id:      i32,
        owner_name:   String,
        owner_link:   String,
        owner_image:  Option<String>,
        attach:       Option<String>,
    ) -> Post {
        let _connection = establish_connection();

        let new_post_form = NewPost {
            content: None,
            community_id:   community_id,
            user_id:        user_id,
            owner_name:     owner_name,
            owner_link:     owner_link,
            owner_image:    owner_image,
            post_list_id:   0,
            types:          "r".to_string(),
            attach:          attach,
            comment_enabled: false,
            created:         chrono::Local::now().naive_utc(),
            comment:         0,
            view:            0,
            repost:          0,
            copy:            0,
            position:        0,
            is_signature:    false,
            parent_id:       None,
            reactions:       0,
        };
        let new_post = diesel::insert_into(schema::posts::table)
            .values(&new_post_form)
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return new_post;
    }
    pub fn copy_item(pk: i32, lists: Vec<i32>) -> bool {
        use crate::schema::posts::dsl::posts;
        use crate::schema::post_lists::dsl::post_lists;

        let _connection = establish_connection();
        let item = posts
            .filter(schema::posts::id.eq(pk))
            .filter(schema::posts::types.eq("a"))
            .load::<Post>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
        let mut count = 0;
        for list_id in lists.iter() {
            count += 1;
            let list = post_lists
                .filter(schema::post_lists::id.eq(list_id))
                .filter(schema::post_lists::types.lt(10))
                .load::<PostList>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            list.create_post (
                item.content.clone(),
                list.user_id,
                list.owner_name.clone(),
                list.owner_link.clone(),
                list.owner_image.clone(),
                None,
                item.attach.clone(),
                item.comment_enabled.clone(),
                item.is_signature.clone(),
                item.parent_id.clone(),
            );
        }
        diesel::update(&item)
          .set(schema::posts::copy.eq(item.copy + count))
          .get_result::<Post>(&_connection)
          .expect("Error.");

        //if item.community_id.is_some() {
        //    let community = item.get_community();
        //    community.plus_posts(count);
        //}
        //else {
        //    let creator = item.get_creator();
        //    creator.plus_posts(count);
        //  }
        return true;
    }

    pub fn edit_post(
        &self,
        content: Option<String>,
        attach: Option<String>,
        comment_enabled: bool,
        is_signature: bool
    ) -> &Post {
        let _connection = establish_connection();

        //let mut _content: Option<String> = None;
        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}

        let edit_post = EditPost {
            content: content,
            attach: attach,
            comment_enabled: comment_enabled,
            is_signature: is_signature,
        };
        diesel::update(self)
            .set(edit_post)
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return self;
    }

    pub fn plus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn plus_reactions(&self, count: i32, _user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::reactions.eq(self.reactions + count))
            .get_result::<Post>(&_connection)
            .expect("Error.");

        //if self.community_id.is_some() {
        //    use crate::models::{create_community_wall, create_community_notify};

        //    let community = self.get_community();
        //    create_community_wall (
        //        &user,
        //        &community,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //    create_community_notify (
        //        &user,
        //        &community,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //}
        //else {
        //    use crate::models::{create_user_wall, create_user_notify};

        //    create_user_wall (
        //        &user,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //    create_user_notify (
        //        &user,
        //        "отреагировал на запись".to_string(),
        //        51,
        //        self.id,
        //        None,
        //        true
        //    );
        //}
    }
    pub fn minus_reactions(&self, count: i32) -> () {
        //use crate::schema::{
        //    notifications::dsl::notifications,
        //    wall_objects::dsl::wall_objects,
        //};

        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::reactions.eq(self.reactions - count))
            .get_result::<Post>(&_connection)
            .expect("Error.");

        //let _q_standalone = "%".to_owned() + &"отреагировал на запись".to_string() + &"%".to_string();
        //diesel::delete (
        //    notifications
        //        .filter(schema::notifications::types.eq(51))
        //        .filter(schema::notifications::object_id.eq(self.id))
        //        .filter(schema::notifications::verb.ilike(&_q_standalone))
        //    )
        //    .execute(&_connection)
        //    .expect("E");

        //diesel::delete (
        //    wall_objects
        //        .filter(schema::wall_objects::types.eq(51))
        //        .filter(schema::wall_objects::object_id.eq(self.id))
        //        .filter(schema::wall_objects::verb.ilike(&_q_standalone))
        //    )
        //    .execute(&_connection)
        //    .expect("E");
    }
    pub fn minus_comments(&self, count: i32) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment - count))
            .get_result::<Post>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn is_open(&self) -> bool {
        return self.types == "a" && self.types == "b";
    }
    pub fn is_deleted(&self) -> bool {
        return self.types == "c";
    }
    pub fn is_closed(&self) -> bool {
        return self.types == "h";
    }
    pub fn is_fixed(&self) -> bool {
        return self.types == "b";
    }
    pub fn is_repost(&self) -> bool {
        return self.types == "r";
    }

    pub fn delete_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "c",
            "b" => "m",
            "f" => "i",
            "g" => "y",
            _ => "c",
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count - 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.minus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.minus_posts(1);
        //}

        //hide_wall_notify_items(51, self.id);
    }
    pub fn restore_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "c" => "a",
            "m" => "b",
            "i" => "f",
            "y" => "g",
            _ => "a",
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count + 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.plus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.plus_posts(1);
        //}

        //show_wall_notify_items(51, self.id);
    }

    pub fn close_item(&self) -> () {
        //use crate::models::hide_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "a" => "h",
            "b" => "n",
            _ => "h",
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count - 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.minus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.minus_posts(1);
        //}

        //hide_wall_notify_items(51, self.id);
    }
    pub fn unclose_item(&self) -> () {
        //use crate::models::show_wall_notify_items;

        let _connection = establish_connection();
        let user_types = &self.types;
        let close_case = match user_types.as_str() {
            "h" => "a",
            "n" => "b",
            _ => "a",
        };
        diesel::update(self)
            .set(schema::posts::types.eq(close_case))
            .get_result::<Post>(&_connection)
            .expect("E");
        let list = self.get_list();
        diesel::update(&list)
            .set(schema::post_lists::count.eq(list.count + 1))
            .get_result::<PostList>(&_connection)
            .expect("E");

        //if self.community_id.is_some() {
        //    let community = self.get_community();
        //    community.plus_posts(1);
        //}
        //else {
        //    let creator = self.get_creator();
        //    creator.plus_posts(1);
        //}

        //show_wall_notify_items(51, self.id);
    }
    pub fn get_format_text(&self) -> String {
        if self.content.is_some() {
            let unwrap = self.content.as_ref().unwrap();
            let split_unwrap: Vec<&str> = unwrap.split(" ").collect();
            if split_unwrap.len() <= 20 {
                return self.content.as_ref().unwrap().to_string();
            }
            else {
                let mut string = String::new();
                for (i, word) in split_unwrap.iter().enumerate() {
                    if i == 20 {
                        string.push_str("<br><a class='pointer show_post_text'>Показать полностью...</a><br><span style='display:none'>");
                    }
                    string.push_str(word);
                    string.push_str(" ");
                }
                return string;
            }
        } else { return "".to_string(); }
    }

    pub fn count_comments(&self) -> String {
        if self.comment == 0 {
            return "".to_string();
        }
        else {
            return self.comment.to_string();
        }
    }

    pub fn count_reposts(&self) -> String {
        if self.repost == 0 {
            return "".to_string();
        }
        else {
            return self.repost.to_string();
        }
    }
    pub fn count_copy(&self) -> String {
        if self.copy == 0 {
            return "".to_string();
        }
        else {
            return ", копировали - ".to_string() + &self.copy.to_string();
        }
    }

    pub fn reposts_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.repost,
            " человек".to_string(),
            " человека".to_string(),
            " человек".to_string(),
        );
    }

    pub fn is_have_reposts(&self) -> bool {
        return self.repost > 0;
    }

    //pub fn fixed_post(&self, user: User) -> bool {
    //    if user.is_can_fixed_post() {
    //        let _connection = establish_connection();
    //        diesel::update(self)
    //            .set(schema::posts::types.eq("b"))
    //            .get_result::<Post>(&_connection)
    //            .expect("E");
    //        return true;
    //    }
    //    return false;
    //}
    pub fn unfixed_post(&self) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::types.eq("a"))
            .get_result::<Post>(&_connection)
            .expect("E");
        return true;
    }
    pub fn get_count_attach(&self) -> String {
        if self.attach.is_some() {
            let self_attach = self.attach.as_deref().unwrap().split(",").collect::<Vec<_>>();
            return "files_".to_string() + &self_attach.len().to_string();
        }
        return "files_0".to_string();
    }

    pub fn reposts(&self, limit: i64, offset: i64) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::parent_id.eq(self.id))
            .filter(schema::posts::types.eq_any(vec!["a", "b"]))
            .limit(limit)
            .offset(offset)
            .load::<Post>(&_connection)
            .expect("E");
    }
    pub fn window_reposts(&self) -> Vec<Post> {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::parent_id.eq(self.id))
            .filter(schema::posts::types.eq_any(vec!["a", "b"]))
            .limit(6)
            .load::<Post>(&_connection)
            .expect("E");
    }

    pub fn count_reactions(&self) -> String {
        if self.reactions == 0 {
            return "".to_string();
        }
        else {
            return self.reactions.to_string();
        }
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
        use crate::schema::post_votes::dsl::post_votes;
        use crate::models::PostVote;

        let _connection = establish_connection();
        let votes = post_votes
            .filter(schema::post_votes::post_id.eq(self.id))
            .load::<PostVote>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for _item in votes.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }

    pub fn is_have_user_reaction(&self, user_id: i32) -> bool {
        return self.reactions_ids().iter().any(|&i| i==user_id);
    }

    pub fn get_user_reaction(&self, user_id: i32) -> i16 {
        use crate::schema::post_votes::dsl::post_votes;
        use crate::models::PostVote;
        // "/static/images/reactions/" + get_user_reaction + ".jpg"

        let _connection = establish_connection();
        let vote = post_votes
            .filter(schema::post_votes::user_id.eq(user_id))
            .filter(schema::post_votes::post_id.eq(self.id))
            .load::<PostVote>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();

        return vote.reaction;
    }

    pub fn change_position(query: Json<Vec<JsonPosition>>) -> () {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        for i in query.iter() {
            let item = posts
                .filter(schema::posts::id.eq(i.key))
                .filter(schema::posts::types.eq("a"))
                .limit(1)
                .load::<Post>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

            diesel::update(&item)
                .set(schema::posts::position.eq(i.value))
                .get_result::<Post>(&_connection)
                .expect("Error.");
        }
    }

    pub fn create_comment(
        &self,
        user_id:    i32,
        owner_name:  String,
        owner_link:  String,
        owner_image: Option<String>,
        attach:     Option<String>,
        parent_id:  Option<i32>,
        content:    Option<String>,
        sticker_id: Option<i32>
    ) -> PostComment {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::posts::comment.eq(self.comment + 1))
            .get_result::<Post>(&_connection)
            .expect("Error.");

        //let mut _content: Option<String> = None;
        //if content.is_some() {
        //    use crate::utils::get_formatted_text;
        //    _content = Some(get_formatted_text(&content.unwrap()));
        //}

        let new_comment_form = NewPostComment {
            post_id:    self.id,
            user_id:    user_id,
            owner_name:  owner_name,
            owner_link:  owner_link,
            owner_image: owner_image,
            sticker_id: sticker_id,
            parent_id:  parent_id,
            content:    content,
            attach:     attach,
            types:      "a".to_string(),
            created:    chrono::Local::now().naive_utc(),
            repost:     0,
            reactions:  0,
        };
        let new_comment = diesel::insert_into(schema::post_comments::table)
            .values(&new_comment_form)
            .get_result::<PostComment>(&_connection)
            .expect("Error.");

        return new_comment;
    }
    pub fn get_parent(&self) -> Post {
        use crate::schema::posts::dsl::posts;

        let _connection = establish_connection();
        return posts
            .filter(schema::posts::id.eq(self.parent_id.unwrap()))
            .filter(schema::posts::types.eq_any(vec!["a", "b"]))
            .load::<Post>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_comments(&self, limit: i64, offset: i64) -> Vec<PostComment> {
        use crate::schema::post_comments::dsl::post_comments;

        let _connection = establish_connection();

        return post_comments
            .filter(schema::post_comments::post_id.eq(self.id))
            .filter(schema::post_comments::types.eq_any(vec!["a","b"]))
            .filter(schema::post_comments::parent_id.is_null())
            .limit(limit)
            .offset(offset)
            .load::<PostComment>(&_connection)
            .expect("E.");
    }
}
