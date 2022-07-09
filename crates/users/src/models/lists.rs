use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    follows,
    friends,
    design_settings,
    user_privates,
    user_profiles,
    user_locations,
    ip_users,
    user_anketas,
    user_delete_anketas,
    user_love_statuss,
    user_partner_ones,
    user_mom_ones,
    user_dad_ones,
    user_brother_sisters,
    user_children_ones,
    user_grandsons_ones,
    user_colleagues_ones,
    user_blocks,
    list_user_communities_keys,
    featured_user_communities,
    news_user_communities,
    notify_user_communities,
    user_notifications,
    user_populate_smiles,
    user_populate_stickers,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use actix_web::web::Json;

/////// Follow //////
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Follow {
    pub id:            i32,
    pub user_id:       i32,
    pub followed_user: i32,
    pub view:          bool,
    pub visited:       i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="follows"]
pub struct NewFollow {
    pub user_id:       i32,
    pub followed_user: i32,
    pub view:          bool,
    pub visited:       i32,
}

/////// Friend //////
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Friend {
    pub id:             i32,
    pub user_id:        i32,
    pub target_user_id: i32,
    pub visited:        i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="friends"]
pub struct NewFriend {
    pub user_id:        i32,
    pub target_user_id: i32,
    pub visited:        i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserProfile {
    pub id:             i32,
    pub user_id:        i32,
    pub posts:          i32,
    pub friends:        i32,
    pub follows:        i32,
    pub communities:    i32,
    pub photos:         i32,
    pub goods:          i32,
    pub docs:           i32,
    pub tracks:         i32,
    pub videos:         i32,
    pub articles:       i32,
    pub planners:       i32,
    pub avatar_id:      Option<i32>,
    pub survey:         i32,
    pub saved_playlist: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_profiles"]
pub struct NewUserProfile {
    pub user_id:        i32,
    pub posts:          i32,
    pub friends:        i32,
    pub follows:        i32,
    pub communities:    i32,
    pub photos:         i32,
    pub goods:          i32,
    pub docs:           i32,
    pub tracks:         i32,
    pub videos:         i32,
    pub articles:       i32,
    pub planners:       i32,
    pub avatar_id:      Option<i32>,
    pub survey:         i32,
    pub saved_playlist: String,
}

/////// UserLocation //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserLocation {
    pub id:         i32,
    pub user_id:    i32,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="user_locations"]
pub struct NewUserLocation {
    pub user_id:    i32,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}

/////// UserLocation //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct IpUser {
    pub id:      i32,
    pub user_id: i32,
    pub ip:      String,
}
#[derive(Deserialize, Insertable)]
#[table_name="ip_users"]
pub struct NewIpUser {
    pub user_id: i32,
    pub ip:      String,
}

/////// UserAnketa //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserAnketa {
    pub id:                    i32,
    pub user_id:               i32,
    pub political_preferences: Option<String>,
    pub worldview:             Option<String>,
    pub mainthing_in_life:     Option<String>,
    pub mainthing_in_people:   Option<String>,
    pub attitude_to_smoking:   Option<String>,
    pub attitude_to_alcohol:   Option<String>,
    pub inspiration:           Option<String>,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_anketas"]
pub struct NewUserAnketa {
    pub user_id: i32,
}

/////// UserDeleteAnketa //////
    // 'a' "У меня есть другая страница",
    // 'b' "Соцсеть отнимает много времени",
    // 'c' "Мало свободы самовыражения",
    // 'd' "Соцсеть плохо защищает данные",
    // 'e' "Соцсеть плохо защищает детей",
    // 'f' "Другая причина",

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserDeleteAnketa {
    pub id:      i32,
    pub user_id: i32,
    pub answer:  String,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_delete_anketas"]
pub struct NewUserDeleteAnketa {
    pub user_id: i32,
    pub answer:  String,
    pub other:   Option<String>,
    pub created: chrono::NaiveDateTime,
}

/////// UserLoveStatus //////

// 'a' "Не выбрано",
// 'b' "Не женат",
// 'c' "Есть подруга",
// 'd' "Помолвлен",
// 'e' "Женат",
// 'f' "В гражданском браке",
// 'g' "Влюблён",
// 'h' "Всё сложно",
// 'i' "В активном поиске",

// 'a' "Не выбрано",
// 'b' "Не женат",
// 'c' "Есть подруга",
// 'd' "Помолвлен",
// 'e' "Женат",
// 'f' "В гражданском браке",
// 'g' "Влюблён",
// 'h' "Всё сложно",
// 'i' "В активном поиске",

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserLoveStatus {
    pub id:             i32,
    pub user_id:        i32,
    pub male_status:    String,
    pub female_status:  String,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_love_statuss"]
pub struct NewUserLoveStatus {
    pub user_id:        i32,
    pub male_status:    String,
    pub female_status:  String,
}

/////// UserPartnerOne //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPartnerOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_partner_ones"]
pub struct NewUserPartnerOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserMomOne //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserMomOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_mom_ones"]
pub struct NewUserMomOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserDadOne //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserDadOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_dad_ones"]
pub struct NewUserDadOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserBrothersSisters //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserBrotherSister {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_brother_sisters"]
pub struct NewUserBrotherSister {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserChildren //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserChildrenOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_children_ones"]
pub struct NewUserChildrenOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserGrandsons //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserGrandsonsOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_grandsons_ones"]
pub struct NewUserGrandsonsOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserColleagues //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserColleaguesOne {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_colleagues_ones"]
pub struct NewUserColleaguesOne {
    pub user_id:   i32,
    pub target_id: i32,
}

/////// UserBlocks //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserBlock {
    pub id:        i32,
    pub user_id:   i32,
    pub target_id: i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_blocks"]
pub struct NewUserBlock {
    pub user_id:   i32,
    pub target_id: i32,
}

//////////////////////////////////////////////////////
/////// ListUC //////
    // 'b' Не активный
    // 'a' Активный список

#[derive(Queryable, Serialize, Deserialize)]
pub struct ListUserCommunitiesKey {
    pub id:     i32,
    pub types:  String,
    pub name:   String,
    pub owner:  i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="list_user_communities_keys"]
pub struct NewListUserCommunitiesKey {
    pub types: String,
    pub name:  String,
    pub owner: i32,
}

/////// FeaturedUC //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct FeaturedUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable)]
#[table_name="featured_user_communities"]
pub struct NewFeaturedUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// NewsUC //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct NewsUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="news_user_communities"]
pub struct NewNewsUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}

/////// NotifyUC //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct NotifyUserCommunitie {
    pub id:           i32,
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="notify_user_communities"]
pub struct NewNotifyUserCommunitie {
    pub owner:        i32,
    pub list_id:      Option<i32>,
    pub user_id:      Option<i32>,
    pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<chrono::NaiveDateTime>,
}
/////====================================////

/////// UserPrivate //////
    // 'a' Все пользователи
    // 'b' Друзья
    // 'c' Друзья и друзья друзей
    // 'd' Только я
    // 'e' Друзья, кроме
    // 'f' Некоторые друзья

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPrivate {
    pub id:                 i32,
    pub user_id:            i32,
    pub can_see_all:        String,
    pub can_see_community:  String,
    pub can_see_info:       String,
    pub can_see_friend:     String,
    pub can_send_message:   String,
    pub can_add_in_chat:    String,
    pub can_see_post:       String,
    pub can_see_photo:      String,
    pub can_see_good:       String,
    pub can_see_video:      String,
    pub can_see_music:      String,
    pub can_see_planner:    String,
    pub can_see_doc:        String,
    pub can_see_survey:     String,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_privates"]
pub struct NewUserPrivate {
    pub user_id:            i32,
    pub can_see_all:        String,
    pub can_see_community:  String,
    pub can_see_info:       String,
    pub can_see_friend:     String,
    pub can_send_message:   String,
    pub can_add_in_chat:    String,
    pub can_see_post:       String,
    pub can_see_photo:      String,
    pub can_see_good:       String,
    pub can_see_video:      String,
    pub can_see_music:      String,
    pub can_see_planner:    String,
    pub can_see_doc:        String,
    pub can_see_survey:     String,
}

/////// UserPopulateSmiles //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPopulateSmile {
    pub id:       i32,
    pub user_id:  i32,
    pub smile_id: i32,
    pub count:    i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_populate_smiles"]
pub struct NewUserPopulateSmile {
    pub user_id:   i32,
    pub smile_id:  i32,
    pub count:     i32,
}

/////// UserPopulateStickers //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserPopulateSticker {
    pub id:         i32,
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_populate_stickers"]
pub struct NewUserPopulateSticker {
    pub user_id:    i32,
    pub sticker_id: i32,
    pub count:      i32,
}

/////// UserNotifications //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct UserNotification {
    pub id:                   i32,
    pub user_id:              i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub user_invite:          bool,
}
#[derive(Deserialize, Insertable)]
#[table_name="user_notifications"]
pub struct NewUserNotification {
    pub user_id:         i32,
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub user_invite:     bool,
}

/////// design_settings //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct DesignSetting {
    pub id:         i32,
    pub user_id:    i32,
    pub background: String,
}
#[derive(Deserialize, Insertable)]
#[table_name="design_settings"]
pub struct NewDesignSetting {
    pub user_id:    i32,
    pub background: String,
}
#[derive(Deserialize, AsChangeset)]
#[table_name="design_settings"]
pub struct EditDesignSetting {
    pub background: String,
}
