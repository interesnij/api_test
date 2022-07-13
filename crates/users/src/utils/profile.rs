use serde::Serialize;


////////
#[derive(Serialize)]
pub struct UserDetailJson {
    pub id:            i32,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         i16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          String,
    pub link:          String, // community.get_link()
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub image:         Option<String>,
    pub birthday:      String,
    pub last_activity: String,
}

////////
#[derive(Serialize)]
pub struct LocationsJson {
    pub locations: Vec<LocationJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
pub struct LocationJson {
    pub city_ru:    String,
    //pub city_en:    String,
    pub region_ru:  String,
    //pub region_en:  String,
    pub country_ru: String,
    //pub country_en: String,
}

////////
#[derive(Serialize)]
pub struct ProfileJson {
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
    pub avatar_id:      i32,
    pub survey:         i32,
    pub saved_playlist: String,
}

////////
#[derive(Serialize)]
pub struct IpsJson {
    pub ips:       Vec<IpJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
pub struct IpJson {
    pub ip:    String,
}

////////
#[derive(Serialize)]
pub struct ListsUserCommunitiesJson {
    pub lists:     Vec<ListUserCommunitiesJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
pub struct ListUserCommunitiesJson {
    pub id:    i32,
    pub name:  String,
    pub types: String,
}

////////
#[derive(Serialize)]
pub struct AnketaJson {
    pub political_preferences: String,
    pub worldview:             String,
    pub mainthing_in_life:     String,
    pub mainthing_in_people:   String,
    pub attitude_to_smoking:   String,
    pub attitude_to_alcohol:   String,
    pub inspiration:           String,
}

////////
#[derive(Serialize)]
pub struct LoveStatusJson {
    pub male_status:   String,
    pub female_status: String,
}


//////// FeaturedUserCommunities, NewsUserCommunities, NotifyUserCommunities
////////
#[derive(Serialize)]
pub struct UniversalUserCommunityKeysJson {
    pub keys:      Vec<UniversalUserCommunityKeyJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
pub struct UniversalUserCommunityKeyJson {
    pub id:           i32,
    pub list_id:      Option<i32>,
    //pub user_id:      Option<i32>,
    //pub community_id: Option<i32>,
    pub mute:         bool,
    pub sleep:        Option<String>,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
}

////////
#[derive(Serialize)]
pub struct DesignSettingsJson {
    pub background: String,
}

////////
#[derive(Serialize)]
pub struct UserPrivateJson {
    pub can_see_all:       String,
    pub can_see_community: String,
    pub can_see_info:      String,
    pub can_see_friend:    String,
    pub can_send_message:  String,
    pub can_add_in_chat:   String,
    pub can_see_post:      String,
    pub can_see_photo:     String,
    pub can_see_good:      String,
    pub can_see_video:     String,
    pub can_see_music:     String,
    pub can_see_planner:   String,
    pub can_see_doc:       String,
    pub can_see_survey:    String,
}

////////
#[derive(Serialize)]
pub struct UserProfileNotificationJson {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
    pub message:              bool,
}

////////
#[derive(Serialize)]
pub struct UserPopulateStickerJson {
    pub user_id:    i32,
    pub sticker_id: i32,
    pub image:      String,
}

////////
#[derive(Serialize)]
pub struct UserPopulateSmileJson {
    pub user_id:  i32,
    pub smile_id: i32,
    pub image:    String,
}

////////
#[derive(Serialize)]
pub struct FriendsVisiblePermJson {
    pub can_see_community:       String,
    pub can_see_info:            String,
    pub can_see_friend:          String,
    pub can_send_message:        String,
    pub can_add_in_chat:         String,
    pub can_see_doc:             String,
    pub can_see_music:           String,
    pub can_see_survey:          String,
    pub can_see_post:            String,
    pub can_see_post_comment:    String,
    pub can_see_photo:           String,
    pub can_see_photo_comment:   String,
    pub can_see_good:            String,
    pub can_see_good_comment:    String,
    pub can_see_video:           String,
    pub can_see_video_comment:   String,
    pub can_see_planner:         String,
    pub can_see_planner_comment: String,
    pub can_see_all:             String,
}

////////
#[derive(Serialize)]
pub struct PhoneCodeJson {
    pub phone: String,
    pub code:  i32,
}

////////
#[derive(Serialize)]
pub struct UserWorkPermJson {
    pub can_copy_post:    String,
    pub can_copy_photo:   String,
    pub can_copy_good:    String,
    pub can_copy_video:   String,
    pub can_copy_planner: String,
    pub can_copy_doc:     String,
    pub can_copy_music:   String,
    pub can_copy_survey:  String,
    pub can_work_post:    String,
    pub can_work_photo:   String,
    pub can_work_good:    String,
    pub can_work_video:   String,
    pub can_work_planner: String,
    pub can_work_doc:     String,
    pub can_work_music:   String,
    pub can_work_survey:  String,
}
