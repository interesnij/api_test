use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    community_categorys,
    community_subcategorys,
    communitys,
    communities_memberships,
    community_infos,
    community_privates,
    community_notifications,
    community_visible_perms,
    community_work_perms,
    community_banner_users,
    community_follows,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use actix_web::web::Json;

/////// CommunityCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityCategory {
    pub id:       i32,
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

impl CommunityCategory {
    pub fn create_category(name: String, avatar: Option<String>,
        position: i16) -> CommunityCategory {

        let _connection = establish_connection();
        let new_form = NewCommunityCategory {
            name: name,
            avatar: avatar,
            position: position,
        };
        let new_cat = diesel::insert_into(schema::community_categorys::table)
            .values(&new_form)
            .get_result::<CommunityCategory>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn create_subcategory(&self, name: String, avatar: Option<String>,
        position: i16) -> CommunitySubcategory {

        let _connection = establish_connection();
        let new_form = NewCommunitySubcategory {
            name:        name,
            category_id: self.id,
            avatar:      avatar,
            position:    position,
        };
        let new_cat = diesel::insert_into(schema::community_subcategorys::table)
            .values(&new_form)
            .get_result::<CommunitySubcategory>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, avatar: Option<String>,
        position: i16) -> &CommunityCategory {
        let _connection = establish_connection();
        let new_form = NewCommunityCategory {
            name:        name,
            avatar:      avatar,
            position:    position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<CommunityCategory>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="community_categorys"]
pub struct NewCommunityCategory {
    pub name:     String,
    pub avatar:   Option<String>,
    pub position: i16,
}

/////// CommunitySubCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunitySubcategory {
    pub id:          i32,
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}

impl CommunitySubcategory {
    pub fn edit_subcategory(&self, name: String, category_id: i32,
        avatar: Option<String>, position: i16) -> &CommunitySubcategory {
        let _connection = establish_connection();
        let new_form = NewCommunitySubcategory {
            name:        name,
            category_id: category_id,
            avatar:      avatar,
            position:    position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<CommunitySubcategory>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="community_subcategorys"]
pub struct NewCommunitySubcategory {
    pub name:        String,
    pub category_id: i32,
    pub avatar:      Option<String>,
    pub position:    i16,
}

/////// Community //////

/////// Тип сообщества //////
    // 1 приватное сообщество
    // 2 закрытое сообщество
    // 3 публичное сообщество
    // 13 удаленное публичное
    // 11 удаленное приватное
    // 12 удаленное закрытое
    // 23 баннер публичный
    // 21 баннер приватный
    // 22 баннер закрытый
    // 33 заблокированное публичное
    // 31 заблокированное приватное
    // 32 заблокированное закрытое
    // 43 приостановленное публичное
    // 41 приостановленное приватное
    // 42 приостановленное закрытое

/////// Статус сообщества //////
    // 'a' стандартное сообщество
    // 'b' детское сообщество
    // 'c' подавшее на идентификацию сообщество
    // 'd' идентификацированное сообщество

/////// Community //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Community {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub types:       i16,
    pub perm:        String,
    pub level:       i16,
    pub link:        String,
    pub b_avatar:    Option<String>,
    pub s_avatar:    Option<String>,
    pub cover:       Option<String>,
    pub category_id: i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="communitys"]
pub struct NewCommunity {
    pub name:        String,
    pub status:      Option<String>,
    pub types:       i16,
    pub link:        String,
    pub perm:        String,
    pub level:       i16,
    pub category_id: i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl Community {
    pub fn get_str_id(&self) -> String {
        return self.id.to_string();
    }
    pub fn is_identified(&self) -> bool {
        return self.perm == "d";
    }
    pub fn is_identified_send(&self) -> bool {
        return self.perm == "c";
    }
    pub fn get_description(&self) -> String {
        return "<a href='".to_string() + &self.link.to_string() + &"' target='_blank'>".to_string() + &self.name + &"</a>".to_string();
    }
    pub fn get_bb_avatar(&self) -> String {
        if self.b_avatar.is_some() {
            return self.b_avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }
    pub fn count_communities() -> usize {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .load::<Community>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_b_avatar(&self) -> String {
        //let avatar_pk = self.get_avatar_pk();
        //if avatar_pk != 0 {
        //    return "<img src='".to_string() + &self.b_avatar.as_ref().unwrap() + &"' class='detail_photo pointer' photo-pk='".to_string() + &avatar_pk.to_string() + &"'>".to_string();
        //}
        //else {
        return "<img src='/static/images/no_img/b_avatar.png' />".to_string();
        //}
    }
    pub fn get_ss_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return self.s_avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/no_img/list.jpg".to_string();
        }
    }

    pub fn get_s_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:30px;width:30px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg class='svg_default_30 svg_default' fill='currentColor' viewBox='0 0 24 24'><rect fill='none' /><g><path d='M4,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C2,12.1,2.9,13,4,13z M5.13,14.1C4.76,14.04,4.39,14,4,14 c-0.99,0-1.93,0.21-2.78,0.58C0.48,14.9,0,15.62,0,16.43V18l4.5,0v-1.61C4.5,15.56,4.73,14.78,5.13,14.1z M20,13c1.1,0,2-0.9,2-2 c0-1.1-0.9-2-2-2s-2,0.9-2,2C18,12.1,18.9,13,20,13z M24,16.43c0-0.81-0.48-1.53-1.22-1.85C21.93,14.21,20.99,14,20,14 c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V18l4.5,0V16.43z M16.24,13.65c-1.17-0.52-2.61-0.9-4.24-0.9 c-1.63,0-3.07,0.39-4.24,0.9C6.68,14.13,6,15.21,6,16.39V18h12v-1.61C18,15.21,17.32,14.13,16.24,13.65z M8.07,16 c0.09-0.23,0.13-0.39,0.91-0.69c0.97-0.38,1.99-0.56,3.02-0.56s2.05,0.18,3.02,0.56c0.77,0.3,0.81,0.46,0.91,0.69H8.07z M12,8 c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,8,12,8 M12,6c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3 C15,7.34,13.66,6,12,6L12,6z'/></g></svg>".to_string();
        }
    }
    pub fn get_40_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:40px;width:40px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg class='svg_default_40 svg_default' fill='currentColor' viewBox='0 0 24 24'><rect fill='none' /><g><path d='M4,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C2,12.1,2.9,13,4,13z M5.13,14.1C4.76,14.04,4.39,14,4,14 c-0.99,0-1.93,0.21-2.78,0.58C0.48,14.9,0,15.62,0,16.43V18l4.5,0v-1.61C4.5,15.56,4.73,14.78,5.13,14.1z M20,13c1.1,0,2-0.9,2-2 c0-1.1-0.9-2-2-2s-2,0.9-2,2C18,12.1,18.9,13,20,13z M24,16.43c0-0.81-0.48-1.53-1.22-1.85C21.93,14.21,20.99,14,20,14 c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V18l4.5,0V16.43z M16.24,13.65c-1.17-0.52-2.61-0.9-4.24-0.9 c-1.63,0-3.07,0.39-4.24,0.9C6.68,14.13,6,15.21,6,16.39V18h12v-1.61C18,15.21,17.32,14.13,16.24,13.65z M8.07,16 c0.09-0.23,0.13-0.39,0.91-0.69c0.97-0.38,1.99-0.56,3.02-0.56s2.05,0.18,3.02,0.56c0.77,0.3,0.81,0.46,0.91,0.69H8.07z M12,8 c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,8,12,8 M12,6c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3 C15,7.34,13.66,6,12,6L12,6z'/></g></svg>".to_string();
        }
    }
    pub fn get_50_avatar(&self) -> String {
        if self.s_avatar.is_some() {
            return "<img style='border-radius:50px;width:50px;' alt='image' src='".to_owned() + &self.s_avatar.as_deref().unwrap().to_string() +  &"' />".to_string();
        }
        else {
            return "<svg class='svg_default_50 svg_default' fill='currentColor' viewBox='0 0 24 24'><rect fill='none' /><g><path d='M4,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C2,12.1,2.9,13,4,13z M5.13,14.1C4.76,14.04,4.39,14,4,14 c-0.99,0-1.93,0.21-2.78,0.58C0.48,14.9,0,15.62,0,16.43V18l4.5,0v-1.61C4.5,15.56,4.73,14.78,5.13,14.1z M20,13c1.1,0,2-0.9,2-2 c0-1.1-0.9-2-2-2s-2,0.9-2,2C18,12.1,18.9,13,20,13z M24,16.43c0-0.81-0.48-1.53-1.22-1.85C21.93,14.21,20.99,14,20,14 c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V18l4.5,0V16.43z M16.24,13.65c-1.17-0.52-2.61-0.9-4.24-0.9 c-1.63,0-3.07,0.39-4.24,0.9C6.68,14.13,6,15.21,6,16.39V18h12v-1.61C18,15.21,17.32,14.13,16.24,13.65z M8.07,16 c0.09-0.23,0.13-0.39,0.91-0.69c0.97-0.38,1.99-0.56,3.02-0.56s2.05,0.18,3.02,0.56c0.77,0.3,0.81,0.46,0.91,0.69H8.07z M12,8 c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,8,12,8 M12,6c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3 C15,7.34,13.66,6,12,6L12,6z'/></g></svg>".to_string();
        }
    }
    pub fn is_community(&self) -> bool {
        return true;
    }
    pub fn get_code(&self) -> String {
        return "com".to_string() + &self.get_str_id();
    }

    pub fn get_slug(&self) -> String {
        return "@".to_string() + &self.link.replace("/", "").to_string();
    }
    pub fn is_have_music(&self) -> bool {
        return self.get_info_model().tracks > 0;
    }
    pub fn is_have_photo(&self) -> bool {
        return self.get_info_model().photos > 0;
    }
    pub fn is_have_video(&self) -> bool {
        return self.get_info_model().videos > 0;
    }
    pub fn is_have_doc(&self) -> bool {
        return self.get_info_model().docs > 0;
    }
    pub fn is_have_good(&self) -> bool {
        return self.get_info_model().goods > 0;
    }
    pub fn is_have_post(&self) -> bool {
        return self.get_info_model().posts > 0;
    }
    pub fn get_info_model(&self) -> CommunityInfo {
        use crate::schema::community_infos::dsl::community_infos;

        let _connection = establish_connection();
        let infos = community_infos
            .filter(schema::community_infos::id.eq(self.id))
            .load::<CommunityInfo>(&_connection)
            .expect("E.");

        if infos.len() > 0 {
            return infos.into_iter().nth(0).unwrap();
        }
        else {
            let _community_info = NewCommunityInfo {
                community_id: self.id,
                posts:        0,
                members:      0,
                photos:       0,
                goods:        0,
                tracks:       0,
                videos:       0,
                docs:         0,
                articles:     0,
                survey:       0,
                planners:     0,
                avatar_id:    None,
            };
            let new_info = diesel::insert_into(schema::community_infos::table)
                .values(&_community_info)
                .get_result::<CommunityInfo>(&_connection)
                .expect("Error saving user_profile.");

            return new_info;
        }
    }
    pub fn plus_photos(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::photos.eq(profile.photos + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_goods(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::goods.eq(profile.goods + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_posts(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::posts.eq(profile.posts + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_videos(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::videos.eq(profile.videos + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_docs(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::docs.eq(profile.docs + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_surveys(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::survey.eq(profile.survey + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_tracks(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::tracks.eq(profile.tracks + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_articles(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::articles.eq(profile.articles + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn plus_members(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::members.eq(profile.members + count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_photos(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::photos.eq(profile.photos - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_surveys(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::survey.eq(profile.survey - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_goods(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::goods.eq(profile.goods - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_posts(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::posts.eq(profile.posts - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_videos(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::videos.eq(profile.videos - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_docs(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::docs.eq(profile.docs - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_tracks(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::tracks.eq(profile.tracks - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_articles(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::articles.eq(profile.articles - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn minus_members(&self, count: i32) -> () {
        let profile = self.get_info_model();
        let _connection = establish_connection();
        diesel::update(&profile)
            .set(schema::community_infos::members.eq(profile.members - count))
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error.");
    }
    pub fn is_deleted(&self) -> bool {
        return self.types > 10 &&  self.types < 20;
    }
    pub fn is_suspended(&self) -> bool {
        return self.types > 40 &&  self.types < 50;
    }
    pub fn is_closed(&self) -> bool {
        return self.types > 30 &&  self.types < 40;
    }
    pub fn is_have_warning_banner(&self) -> bool {
        return self.types > 20 &&  self.types < 30;
    }
    pub fn is_standart(&self) -> bool {
        return self.perm == "a".to_string();
    }
    pub fn is_child(&self) -> bool {
        return self.perm == "b".to_string();
    }
    pub fn is_private(&self) -> bool {
        return self.types == 1;
    }
    pub fn is_close(&self) -> bool {
        return self.types == 2;
    }
    pub fn is_public(&self) -> bool {
        return self.types == 3;
    }
    pub fn is_open(&self) -> bool {
        return self.types < 10;
    }

    pub fn create_banned_user (
        &self,
        user_id:     i32,
        owner_name:  String,
        owner_link:  String,
        owner_image: Option<String>,
    ) -> () {
        let _connection = establish_connection();
        let new_banned_user = NewCommunityBannerUser {
            user_id:      user_id,
            community_id: self.id,
            owner_name:   owner_name,
            owner_link:   owner_link,
            owner_image:  owner_image,
        };
        diesel::insert_into(schema::community_banner_users::table)
            .values(&new_banned_user)
            .get_result::<CommunityBannerUser>(&_connection)
            .expect("Error.");
    }
    pub fn delete_banned_user(&self, user_id: i32) -> () {
        use crate::schema::community_banner_users::dsl::community_banner_users;

        let _connection = establish_connection();
        diesel::delete(community_banner_users
                .filter(schema::community_banner_users::community_id.eq(self.id))
                .filter(schema::community_banner_users::user_id.eq(user_id))
            )
            .execute(&_connection)
            .expect("E");
    }

    pub fn create_community (
        name: String,
        category_id: i32,
        user_id: i32,
        types: i16,
        owner_name: String,
        owner_link: String,
        owner_image: Option<String>
    ) -> String {

        let _connection = establish_connection();
        let count = Community::count_communities() + 1;
        let link = "/public".to_string() + &count.to_string() + &"/".to_string();
        let new_community_form = NewCommunity {
                name:        name,
                status:      None,
                types:       types,
                link:        link,
                perm:        "a".to_string(),
                level:       100,
                category_id: category_id,
                user_id:     user_id,
                created:     chrono::Local::now().naive_utc(),
            };
        let new_community = diesel::insert_into(schema::communitys::table)
            .values(&new_community_form)
            .get_result::<Community>(&_connection)
            .expect("Error.");

        let community_id = new_community.id;

        // записываем профиль нового пользователя
        let _community_info = NewCommunityInfo {
            community_id: community_id,
            posts:        0,
            members:      0,
            photos:       0,
            goods:        0,
            tracks:       0,
            videos:       0,
            docs:         0,
            articles:     0,
            survey:       0,
            planners:     0,
            avatar_id:    None,
        };
        diesel::insert_into(schema::community_infos::table)
            .values(&_community_info)
            .get_result::<CommunityInfo>(&_connection)
            .expect("Error saving user_profile.");

        // записываем приватность нового пользователя
        let _private = NewCommunityPrivate {
            community_id:     community_id,
            can_see_member:   "a".to_string(),
            can_see_info:     "a".to_string(),
            can_send_message: "a".to_string(),
            can_see_post:     "a".to_string(),
            can_see_photo:    "a".to_string(),
            can_see_good:     "a".to_string(),
            can_see_video:    "a".to_string(),
            can_see_music:    "a".to_string(),
            can_see_planner:  "a".to_string(),
            can_see_doc:      "a".to_string(),
            can_see_survey:   "a".to_string(),
            can_see_settings: "c".to_string(),
            can_see_log:      "c".to_string(),
            can_see_stat:     "c".to_string(),
            can_see_forum:    "a".to_string(),
            owner_name:       owner_name,
            owner_link:       owner_link,
            owner_image:      owner_image
        };
        diesel::insert_into(schema::community_privates::table)
            .values(&_private)
            .get_result::<CommunityPrivate>(&_connection)
            .expect("Error saving community_private.");

        // записываем уведомления профиля нового пользователя
        let _community_notification = NewCommunityNotification {
            community_id:         community_id,
            connection_request:   true,
            connection_confirmed: true,
            community_invite:     true,
        };
        diesel::insert_into(schema::community_notifications::table)
            .values(&_community_notification)
            .get_result::<CommunityNotification>(&_connection)
            .expect("Error saving community_notification.");

        CommunitiesMembership::create_membership (
            user_id,
            &new_community,
            true,
            false,
            false,
            false,
            owner_name,
            owner_link,
            owner_image
        );
        return new_community.link;
    }

    pub fn count_goods(&self) -> i32 {
        return self.get_info_model().goods;
    }
    pub fn count_goods_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_goods(),
            " товар".to_string(),
            " товара".to_string(),
            " товаров".to_string(),
        );
    }
    pub fn count_goods_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_goods(),
            " товар".to_string(),
            " товара".to_string(),
            " товаров".to_string(),
        );
    }

    pub fn count_tracks(&self) -> i32 {
        return self.get_info_model().tracks;
    }
    pub fn count_tracks_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_tracks(),
            " аудиозапись".to_string(),
            " аудиозаписи".to_string(),
            " аудиозаписей".to_string(),
        );
    }
    pub fn count_tracks_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_tracks(),
            " аудиозапись".to_string(),
            " аудиозаписи".to_string(),
            " аудиозаписей".to_string(),
        );
    }

    pub fn count_photos(&self) -> i32 {
        return self.get_info_model().photos;
    }
    pub fn count_photos_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_photos(),
            " фотография".to_string(),
            " фотографии".to_string(),
            " фотографий".to_string(),
        );
    }
    pub fn count_photos_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_photos(),
            " фотография".to_string(),
            " фотографии".to_string(),
            " фотографий".to_string(),
        );
    }

    pub fn count_docs(&self) -> i32 {
        return self.get_info_model().docs;
    }
    pub fn count_docs_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_docs(),
            " документ".to_string(),
            " документа".to_string(),
            " документов".to_string(),
        );
    }
    pub fn count_docs_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_docs(),
            " документ".to_string(),
            " документа".to_string(),
            " документов".to_string(),
        );
    }

    pub fn count_posts(&self) -> i32 {
        return self.get_info_model().posts;
    }
    pub fn count_posts_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }
    pub fn count_posts_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_posts(),
            " запись".to_string(),
            " записи".to_string(),
            " записей".to_string(),
        );
    }

    pub fn count_articles(&self) -> i32 {
        return self.get_info_model().articles;
    }
    pub fn count_articles_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_articles(),
            " статья".to_string(),
            " статьи".to_string(),
            " статей".to_string(),
        );
    }
    pub fn count_articles_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_articles(),
            " статья".to_string(),
            " статьи".to_string(),
            " статей".to_string(),
        );
    }

    pub fn count_videos(&self) -> i32 {
        return self.get_info_model().videos;
    }
    pub fn count_videos_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru(
            self.count_videos(),
            " видеозапись".to_string(),
            " видеозаписи".to_string(),
            " видеозаписей".to_string(),
        );
    }
    pub fn count_videos_ru_alt(&self) -> String {
        use crate::utils::get_count_for_ru_alt;

        return get_count_for_ru_alt(
            self.count_videos(),
            " видеозапись".to_string(),
            " видеозаписи".to_string(),
            " видеозаписей".to_string(),
        );
    }

    pub fn count_members(&self) -> i32 {
        return self.get_info_model().members;
    }
    //pub fn count_members_ru(&self) -> String {
    //    use crate::utils::get_count_for_ru;

    //    return get_count_for_ru(
    //        self.count_members(),
    //        " подписчик".to_string(),
    //        " подписчика".to_string(),
    //        " подписчиков".to_string(),
    //    );
    //}
    //pub fn count_members_ru_alt(&self) -> String {
    //    use crate::utils::get_count_for_ru_alt;

    //    return get_count_for_ru_alt(
    //        self.count_members(),
    //        " подписчик".to_string(),
    //        " подписчика".to_string(),
    //        " подписчиков".to_string(),
    //    );
    //}

    pub fn create_administrator(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_administrator.eq(true))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn create_editor(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_editor.eq(true))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn create_moderator(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_moderator.eq(true))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn create_advertisor(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_advertiser.eq(true))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_administrator(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_administrator.eq(false))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_editor(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_editor.eq(false))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_moderator(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_moderator.eq(false))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_advertiser(&self, user_id: i32) -> bool {
        use crate::schema::communities_memberships::dsl::communities_memberships;
        if !self.get_members_ids().iter().any(|&i| i==user_id) {
            return false;
        }
        let _connection = establish_connection();
        let member = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::user_id.eq(user_id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        diesel::update(&member[0])
            .set(schema::communities_memberships::is_advertiser.eq(false))
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("Error.");
        return true;
    }

    pub fn get_members_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_6_members_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .limit(6)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_staff_users_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            if _item.is_administrator || _item.is_moderator || _item.is_editor || _item.is_advertiser {
                stack.push(_item.user_id);
            }
        };
        return stack;
    }
    pub fn get_administrators_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_administrator.eq(true))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_moderators_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_moderator.eq(true))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_editors_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_editor.eq(true))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_advertisers_ids(&self) -> Vec<i32> {
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_advertiser.eq(true))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_info_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_info.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_info_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_info.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_info_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_info_exclude_users_ids());
    //}
    //pub fn get_can_see_info_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_info_include_users_ids());
    //}

    pub fn get_can_see_member_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_member.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_member_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_member.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_member_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_member_exclude_users_ids());
    //}
    //pub fn get_can_see_member_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_member_include_users_ids());
    //}

    pub fn get_can_send_message_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_send_message.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_send_message_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_send_message.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_send_message_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_send_message_exclude_users_ids());
    //}
    //pub fn get_can_send_message_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_send_message_include_users_ids());
    //}

    pub fn get_can_see_doc_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_doc.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_doc_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_doc.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_doc_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_doc_exclude_users_ids());
    //}
    //pub fn get_can_see_doc_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_doc_include_users_ids());
    //}

    pub fn get_can_see_music_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_music.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_music_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_music.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_music_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_music_exclude_users_ids());
    //}
    //pub fn get_can_see_music_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_music_include_users_ids());
    //}

    pub fn get_can_see_survey_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_survey.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_survey_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_survey.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_survey_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_survey_exclude_users_ids());
    //}
    //pub fn get_can_see_survey_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_survey_include_users_ids());
    //}

    pub fn get_can_see_post_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_post.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_post_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_post.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_post_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_post_exclude_users_ids());
    //}
    //pub fn get_can_see_post_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_post_include_users_ids());
    //}

    pub fn get_can_see_photo_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_photo.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_photo_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_photo.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_photo_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_photo_exclude_users_ids());
    //}
    //pub fn get_can_see_photo_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_photo_include_users_ids());
    //}

    pub fn get_can_see_good_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_good.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_good_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_good.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_good_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_good_exclude_users_ids());
    //}
    //pub fn get_can_see_good_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_good_include_users_ids());
    //}

    pub fn get_can_see_video_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_video.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_video_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_video.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_video_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_video_exclude_users_ids());
    //}
    //pub fn get_can_see_video_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_video_include_users_ids());
    //}

    pub fn get_can_see_planner_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_planner.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_planner_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_planner.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_planner_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_planner_exclude_users_ids());
    //}
    //pub fn get_can_see_planner_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_planner_include_users_ids());
    //}

    pub fn get_can_see_forum_exclude_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_forum.eq("b"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    pub fn get_can_see_forum_include_users_ids(&self) -> Vec<i32> {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;

        let _connection = establish_connection();
        let items = community_visible_perms
            .filter(schema::community_visible_perms::user_id.eq_any(self.get_members_ids()))
            .filter(schema::community_visible_perms::can_see_forum.eq("a"))
            .load::<CommunityVisiblePerm>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        return stack;
    }
    //pub fn get_can_see_forum_exclude_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_forum_exclude_users_ids());
    //}
    //pub fn get_can_see_forum_include_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_can_see_forum_include_users_ids());
    //}

    //pub fn get_members(&self, limit: i64, offset: i64) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    use crate::schema::communities_memberships::dsl::communities_memberships;

    //    let _connection = establish_connection();
    //    let items = communities_memberships
    //        .filter(schema::communities_memberships::community_id.eq(self.id))
    //        .limit(limit)
    //        .offset(offset)
    //        .load::<CommunitiesMembership>(&_connection)
    //        .expect("E");

    //    let mut stack = Vec::new();
    //    for _item in items.iter() {
    //        stack.push(_item.user_id);
    //    };
    //    return get_users_from_ids(stack);
    //}
    //pub fn get_6_members(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_6_members_ids());
    //}
    pub fn get_administrators(&self, limit: i64, offset: i64) -> Vec<i32> {
        //use crate::utils::get_users_from_ids;
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_administrator.eq(true))
            .limit(limit)
            .offset(offset)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        //return get_users_from_ids(stack);
        return stack;
    }
    pub fn get_editors(&self, limit: i64, offset: i64) -> Vec<i32> {
        //use crate::utils::get_users_from_ids;
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_editor.eq(true))
            .limit(limit)
            .offset(offset)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        //return get_users_from_ids(stack);
        return stack;
    }
    pub fn get_moderators(&self, limit: i64, offset: i64) -> Vec<i32> {
        //use crate::utils::get_users_from_ids;
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_moderator.eq(true))
            .limit(limit)
            .offset(offset)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        //return get_users_from_ids(stack);
        return stack;
    }
    pub fn get_advertisers(&self, limit: i64, offset: i64) -> Vec<i32> {
        //use crate::utils::get_users_from_ids;
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let items = communities_memberships
            .filter(schema::communities_memberships::community_id.eq(self.id))
            .filter(schema::communities_memberships::is_advertiser.eq(true))
            .limit(limit)
            .offset(offset)
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in items.iter() {
            stack.push(_item.user_id);
        };
        //return get_users_from_ids(stack);
        return stack;
    }
    //pub fn get_staff_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;
    //    return get_users_from_ids(self.get_staff_users_ids());
    //}

    pub fn get_private_model(&self) -> CommunityPrivate {
        use crate::schema::community_privates::dsl::community_privates;

        let _connection = establish_connection();
        return community_privates
            .filter(schema::community_privates::community_id.eq(self.id))
            .load::<CommunityPrivate>(&_connection)
            .expect("E.")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn is_user_can_see_info(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_info;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_member(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_member;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_send_message(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_send_message;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_doc(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_doc;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_music(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_music;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_survey(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_survey;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_post(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_post;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_photo(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_photo;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }

    pub fn is_user_can_see_good(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_good;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }

    pub fn is_user_can_see_video(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_video;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }
    pub fn is_user_can_see_planner(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_planner;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }

    pub fn is_user_can_see_forum(&self, user_id: i32) -> bool {
        let private = self.get_private_model();
        let char = private.can_see_forum;
        return match char.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "h" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "i" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
    }

    pub fn is_anon_user_can_see_info(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_info == "a";
    }
    pub fn is_anon_user_can_see_member(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_member == "a";
    }

    pub fn is_anon_user_can_send_message(&self) -> bool {
        let private = self.get_private_model();
        return private.can_send_message == "a";
    }
    pub fn is_anon_user_can_see_doc(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_doc == "a";
    }
    pub fn is_anon_user_can_see_music(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_music == "a";
    }
    pub fn is_anon_user_can_see_survey(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_survey == "a";
    }
    pub fn is_anon_user_can_see_post(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_post == "a";
    }
    pub fn is_anon_user_can_see_photo(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_photo == "a";
    }
    pub fn is_anon_user_can_see_good(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_good == "a";
    }
    pub fn is_anon_user_can_see_video(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_video == "a";
    }

    pub fn is_anon_user_can_see_planner(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_planner == "a";
    }
    pub fn is_anon_user_can_see_forum(&self) -> bool {
        let private = self.get_private_model();
        return private.can_see_forum == "a";
    }

    pub fn get_community_all_can_see(&self, user_id: i32) -> Vec<bool> {
        if self.id == self.user_id {
            // 14
            return vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true];
        }
        let private = self.get_private_model();

        let mut bool_stack = Vec::new();

        let can_see_info = private.can_see_info;
        let bool_can_see_info = match can_see_info.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_info_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_info_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_info);

        let can_see_member = private.can_see_member;
        let bool_can_see_member = match can_see_member.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_member_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_member_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_member);

        let can_send_message = private.can_send_message;
        let bool_can_send_message = match can_send_message.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_send_message_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_send_message_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_send_message);

        let can_see_doc = private.can_see_doc;
        let bool_can_see_doc = match can_see_doc.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_doc_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_doc_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_doc);

        let can_see_music = private.can_see_music;
        let bool_can_see_music = match can_see_music.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_music_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_music_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_music);

        let can_see_survey = private.can_see_survey;
        let bool_can_see_survey = match can_see_survey.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_survey_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_survey_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_survey);

        let can_see_post = private.can_see_post;
        let bool_can_see_post = match can_see_post.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_post_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_post_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_post);

        let can_see_photo = private.can_see_photo;
        let bool_can_see_photo = match can_see_photo.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_photo_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_photo_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_photo);

        let can_see_good = private.can_see_good;
        let bool_can_see_good = match can_see_good.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_good_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_good_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_good);

        let can_see_video = private.can_see_video;
        let bool_can_see_video = match can_see_video.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_video_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_video_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_video);

        let can_see_planner = private.can_see_planner;
        let bool_can_see_planner = match can_see_planner.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_planner_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_planner_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_planner);

        let can_see_forum = private.can_see_forum;
        let bool_can_see_forum = match can_see_forum.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_forum_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_forum_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_forum);

        let can_see_stat = private.can_see_stat;
        let bool_can_see_stat = match can_see_stat.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_forum_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_forum_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_stat);

        let can_see_settings = private.can_see_settings;
        let bool_can_see_settings = match can_see_settings.as_str() {
            "a" => true,
            "b" => self.get_members_ids().iter().any(|&i| i==user_id),
            "c" => self.get_staff_users_ids().iter().any(|&i| i==user_id),
            "d" => self.get_administrators_ids().iter().any(|&i| i==user_id),
            "e" => self.user_id == user_id,
            "f" => !self.get_can_see_forum_exclude_users_ids().iter().any(|&i| i==user_id),
            "g" => self.get_can_see_forum_include_users_ids().iter().any(|&i| i==user_id),
            _ => false,
        };
        bool_stack.push(bool_can_see_settings);

        return bool_stack;
    }
    pub fn get_anon_community_all_can_see(&self) -> Vec<bool> {
        if self.id == self.user_id {
            // 13
            return vec![true, true, true, true, true, true, true, true, true, true, true, true, true];
        }
        let private = self.get_private_model();

        let mut bool_stack = Vec::new();

        let can_see_info = private.can_see_info;
        let bool_can_see_info = match can_see_info.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_info);

        let can_see_member = private.can_see_member;
        let bool_can_see_member = match can_see_member.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_member);

        let can_see_doc = private.can_see_doc;
        let bool_can_see_doc = match can_see_doc.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_doc);

        let can_see_music = private.can_see_music;
        let bool_can_see_music = match can_see_music.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_music);

        let can_see_survey = private.can_see_survey;
        let bool_can_see_survey = match can_see_survey.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_survey);

        let can_see_post = private.can_see_post;
        let bool_can_see_post = match can_see_post.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_post);

        let can_see_photo = private.can_see_photo;
        let bool_can_see_photo = match can_see_photo.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_photo);

        let can_see_good = private.can_see_good;
        let bool_can_see_good = match can_see_good.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_good);

        let can_see_video = private.can_see_video;
        let bool_can_see_video = match can_see_video.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_video);

        let can_see_planner = private.can_see_planner;
        let bool_can_see_planner = match can_see_planner.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_planner);

        let can_see_forum = private.can_see_forum;
        let bool_can_see_forum = match can_see_forum.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_forum);

        let can_see_stat = private.can_see_stat;
        let bool_can_see_stat = match can_see_stat.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_stat);

        let can_see_settings = private.can_see_settings;
        let bool_can_see_settings = match can_see_settings.as_str() {
            "a" => true,
            _ => false,
        };
        bool_stack.push(bool_can_see_settings);

        return bool_stack;
    }

    pub fn get_follows_users(&self, limit: i64, offset: i64) -> Vec<i32> {
        use crate::schema::community_follows::dsl::community_follows;
        //use crate::utils::get_users_from_ids;

        let _connection = establish_connection();
        let follows = community_follows
            .filter(schema::community_follows::community_id.eq(self.id))
            .limit(limit)
            .offset(offset)
            .load::<CommunityFollow>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for _item in follows.iter() {
            stack.push(_item.user_id);
        };
        //return get_users_from_ids(stack);
        return stack;
    }
    pub fn get_banned_user(&self, limit: i64, offset: i64) -> Vec<i32> {
        use crate::schema::community_banner_users::dsl::community_banner_users;
        //use crate::utils::get_users_from_ids;

        let _connection = establish_connection();

        let banner_users = community_banner_users
            .filter(schema::community_banner_users::community_id.eq(self.id))
            .limit(limit)
            .offset(offset)
            .load::<CommunityBannerUser>(&_connection)
            .expect("E");

        let mut stack = Vec::new();
        for _item in banner_users.iter() {
            stack.push(_item.user_id);
        };
        //return get_users_from_ids(stack);
        return stack;
    }


    pub fn set_friends_visible_perms(&self, action: String, users: String, types: String) -> bool {
        use crate::schema::community_visible_perms::dsl::community_visible_perms;
        use crate::schema::communities_memberships::dsl::communities_memberships;

        let _connection = establish_connection();
        let mut users_ids = Vec::new();
        let v: Vec<&str> = users.split(", ").collect();
        for item in v.iter() {
            if !item.is_empty() {
                let pk: i32 = item.parse().unwrap();
                users_ids.push(pk);
            }
        }

        let _members = communities_memberships
            .filter(schema::communities_memberships::user_id.eq_any(&users_ids))
            .load::<CommunitiesMembership>(&_connection)
            .expect("E");
        let mut members_stack = Vec::new();
        for _item in _members.iter() {
            members_stack.push(_item.user_id);
        };
        diesel::delete(community_visible_perms.filter(schema::community_visible_perms::user_id.eq_any(members_stack))).execute(&_connection).expect("E");

        if types == "can_see_info".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_info(
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_member".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_member(
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_send_message".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_send_message(
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_doc".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_doc(
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_music".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_music (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_survey".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_survey (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_post".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_post (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_photo".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_photo (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_good".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_good (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_video".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_video (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_planner".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_planner (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }
        else if types == "can_see_forum".to_string() {
            for user_id in users_ids.iter() {
                let _new_perm = NewCommunityVisiblePerm::add_can_see_forum (
                    *user_id,
                    action.clone(),
                );
                diesel::insert_into(schema::community_visible_perms::table)
                    .values(&_new_perm)
                    .get_result::<CommunityVisiblePerm>(&_connection)
                    .expect("Error.");
            }
        }

        return true;
    }
}


/////// CommunityMembership //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunitiesMembership {
    pub id:               i32,
    pub user_id:          i32,
    pub community_id:     i32,
    pub is_administrator: bool,
    pub is_moderator:     bool,
    pub is_editor:        bool,
    pub is_advertiser:    bool,
    pub created:          chrono::NaiveDateTime,
    pub visited:          i32,
    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="communities_memberships"]
pub struct NewCommunitiesMembership {
    pub user_id:          i32,
    pub community_id:     i32,
    pub is_administrator: bool,
    pub is_moderator:     bool,
    pub is_editor:        bool,
    pub is_advertiser:    bool,
    pub created:          chrono::NaiveDateTime,
    pub visited:          i32,
    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
}
impl CommunitiesMembership {
    pub fn create_membership (
        user_id: i32,
        community: &Community,
        is_administrator: bool,
        is_editor: bool,
        is_advertiser: bool,
        is_moderator: bool,
        owner_name: String,
        owner_link: String,
        owner_image: Option<String>
    ) -> CommunitiesMembership {
        let _connection = establish_connection();

        let new_member_form = NewCommunitiesMembership {
            user_id:          user_id,
            community_id:     community.id,
            is_administrator: is_administrator,
            is_moderator:     is_moderator,
            is_editor:        is_editor,
            is_advertiser:    is_advertiser,
            created:          chrono::Local::now().naive_utc(),
            visited:          0,
            owner_name:       owner_name,
            owner_link:       owner_link,
            owner_image:      owner_image,
        };
        let new_member = diesel::insert_into(schema::communities_memberships::table)
            .values(&new_member_form)
            .get_result::<CommunitiesMembership>(&_connection)
            .expect("E.");

        //if is_administrator || is_editor || is_moderator {
        //    community.add_notify_subscriber(user.id);
        //}
        //community.add_new_subscriber(user.id);
        community.plus_members(1);
        //user.plus_communities(1);
        //user.plus_community_visited(community.id);
        return new_member;
    }
}

/////// CommunityInfo //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityInfo {
    pub id:           i32,
    pub community_id: i32,
    pub posts:        i32,
    pub members:      i32,
    pub photos:       i32,
    pub goods:        i32,
    pub tracks:       i32,
    pub videos:       i32,
    pub docs:         i32,
    pub articles:     i32,
    pub survey:       i32,
    pub planners:     i32,
    pub avatar_id:    Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_infos"]
pub struct NewCommunityInfo {
    pub community_id: i32,
    pub posts:        i32,
    pub members:      i32,
    pub photos:       i32,
    pub goods:        i32,
    pub tracks:       i32,
    pub videos:       i32,
    pub docs:         i32,
    pub articles:     i32,
    pub survey:       i32,
    pub planners:     i32,
    pub avatar_id:    Option<i32>,
}

/////// CommunityPrivate //////
    // 'a' Все пользователи
    // 'b' Подписчики
    // 'c' Персонал
    // 'd' Администраторы
    // 'e' Владелец сообщества
    // 'h' Подписчики, кроме
    // 'i' Некоторые подписчики

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityPrivate {
    pub id:               i32,
    pub community_id:     i32,
    pub can_see_member:   String,
    pub can_see_info:     String,
    pub can_send_message: String,
    pub can_see_post:     String,
    pub can_see_photo:    String,
    pub can_see_good:     String,
    pub can_see_video:    String,
    pub can_see_music:    String,
    pub can_see_planner:  String,
    pub can_see_doc:      String,
    pub can_see_survey:   String,
    pub can_see_settings: String,
    pub can_see_log:      String,
    pub can_see_stat:     String,
    pub can_see_forum:    String,
    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_privates"]
pub struct NewCommunityPrivate {
    pub community_id:     i32,
    pub can_see_member:   String,
    pub can_see_info:     String,
    pub can_send_message: String,
    pub can_see_post:     String,
    pub can_see_photo:    String,
    pub can_see_good:     String,
    pub can_see_video:    String,
    pub can_see_music:    String,
    pub can_see_planner:  String,
    pub can_see_doc:      String,
    pub can_see_survey:   String,
    pub can_see_settings: String,
    pub can_see_log:      String,
    pub can_see_stat:     String,
    pub can_see_forum:    String,
    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
}

/////// CommunityNotifications //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityNotification {
    pub id:                   i32,
    pub community_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_notifications"]
pub struct NewCommunityNotification {
    pub community_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityVisiblePerm {
    pub id:                      i32,
    pub user_id:                 i32,
    pub can_see_info:            Option<String>,
    pub can_see_community:       Option<String>,
    pub can_see_member:          Option<String>,
    pub can_send_message:        Option<String>,
    pub can_add_in_chat:         Option<String>,
    pub can_see_doc:             Option<String>,
    pub can_see_music:           Option<String>,
    pub can_see_survey:          Option<String>,
    pub can_see_post:            Option<String>,
    pub can_see_post_comment:    Option<String>,
    pub can_see_photo:           Option<String>,
    pub can_see_photo_comment:   Option<String>,
    pub can_see_good:            Option<String>,
    pub can_see_good_comment:    Option<String>,
    pub can_see_video:           Option<String>,
    pub can_see_video_comment:   Option<String>,
    pub can_see_planner:         Option<String>,
    pub can_see_planner_comment: Option<String>,
    pub can_see_forum:           Option<String>,
    pub can_see_forum_comment:   Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_visible_perms"]
pub struct NewCommunityVisiblePerm {
    pub user_id:                 i32,

    pub can_see_info:            Option<String>,
    pub can_see_community:       Option<String>,
    pub can_see_member:          Option<String>,
    pub can_send_message:        Option<String>,
    pub can_add_in_chat:         Option<String>,
    pub can_see_doc:             Option<String>,
    pub can_see_music:           Option<String>,
    pub can_see_survey:          Option<String>,
    pub can_see_post:            Option<String>,
    pub can_see_post_comment:    Option<String>,
    pub can_see_photo:           Option<String>,
    pub can_see_photo_comment:   Option<String>,
    pub can_see_good:            Option<String>,
    pub can_see_good_comment:    Option<String>,
    pub can_see_video:           Option<String>,
    pub can_see_video_comment:   Option<String>,
    pub can_see_planner:         Option<String>,
    pub can_see_planner_comment: Option<String>,
    pub can_see_forum:           Option<String>,
    pub can_see_forum_comment:   Option<String>,
}

impl NewCommunityVisiblePerm {
    pub fn add_can_see_info(user_id: i32, can_see_info: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            Some(can_see_info),
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_member(user_id: i32, can_see_member: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          Some(can_see_member),
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_send_message(user_id: i32, can_send_message: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        Some(can_send_message),
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_doc(user_id: i32, can_see_doc: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             Some(can_see_doc),
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_music(user_id: i32, can_see_music: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           Some(can_see_music),
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_survey(user_id: i32, can_see_survey: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          Some(can_see_survey),
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_post(user_id: i32, can_see_post: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            Some(can_see_post),
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_photo(user_id: i32, can_see_photo: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           Some(can_see_photo),
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_good(user_id: i32, can_see_good: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            Some(can_see_good),
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_video(user_id: i32, can_see_video: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           Some(can_see_video),
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_planner(user_id: i32, can_see_planner: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         Some(can_see_planner),
            can_see_planner_comment: None,
            can_see_forum:           None,
            can_see_forum_comment:   None,
        }
    }
    pub fn add_can_see_forum(user_id: i32, can_see_forum: String) -> Self {
        NewCommunityVisiblePerm {
            user_id:                 user_id,
            can_see_info:            None,
            can_see_community:       None,
            can_see_member:          None,
            can_send_message:        None,
            can_add_in_chat:         None,
            can_see_doc:             None,
            can_see_music:           None,
            can_see_survey:          None,
            can_see_post:            None,
            can_see_post_comment:    None,
            can_see_photo:           None,
            can_see_photo_comment:   None,
            can_see_good:            None,
            can_see_good_comment:    None,
            can_see_video:           None,
            can_see_video_comment:   None,
            can_see_planner:         None,
            can_see_planner_comment: None,
            can_see_forum:           Some(can_see_forum),
            can_see_forum_comment:   None,
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityWorkPerm {
    pub id:               i32,
    pub user_id:          i32,

    pub can_copy_post:    Option<String>,
    pub can_copy_photo:   Option<String>,
    pub can_copy_good:    Option<String>,
    pub can_copy_video:   Option<String>,
    pub can_copy_planner: Option<String>,
    pub can_copy_doc:     Option<String>,
    pub can_copy_music:   Option<String>,
    pub can_copy_survey:  Option<String>,

    pub can_work_post:    Option<String>,
    pub can_work_photo:   Option<String>,
    pub can_work_good:    Option<String>,
    pub can_work_video:   Option<String>,
    pub can_work_planner: Option<String>,
    pub can_work_doc:     Option<String>,
    pub can_work_music:   Option<String>,
    pub can_work_survey:  Option<String>,
    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="community_work_perms"]
pub struct NewCommunityWorkPerm {
    pub id:               i32,
    pub user_id:          i32,

    pub can_copy_post:    Option<String>,
    pub can_copy_photo:   Option<String>,
    pub can_copy_good:    Option<String>,
    pub can_copy_video:   Option<String>,
    pub can_copy_planner: Option<String>,
    pub can_copy_doc:     Option<String>,
    pub can_copy_music:   Option<String>,
    pub can_copy_survey:  Option<String>,

    pub can_work_post:    Option<String>,
    pub can_work_photo:   Option<String>,
    pub can_work_good:    Option<String>,
    pub can_work_video:   Option<String>,
    pub can_work_planner: Option<String>,
    pub can_work_doc:     Option<String>,
    pub can_work_music:   Option<String>,
    pub can_work_survey:  Option<String>,

    pub owner_name:       String,
    pub owner_link:       String,
    pub owner_image:      Option<String>,
}

/////// CommunityBannerUser //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct CommunityBannerUser {
    pub id:           i32,
    pub community_id: i32,
    pub user_id:      i32,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_banner_users"]
pub struct NewCommunityBannerUser {
    pub community_id: i32,
    pub user_id:      i32,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
}

/////// CommunityFollow //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CommunityFollow {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: i32,
    pub view:         bool,
    pub visited:      i32,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="community_follows"]
pub struct NewCommunityFollow {
    pub user_id:      i32,
    pub community_id: i32,
    pub view:         bool,
    pub visited:      i32,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
}
