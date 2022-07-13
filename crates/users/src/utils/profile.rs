use serde::Serialize;


////////
#[derive(Serialize)]
pub struct LocationsJson {
    pub locations: Vec<LocationJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
pub struct LocationJson {
    pub city_ru:    String,
    pub city_en:    String,
    pub region_ru:  String,
    pub region_en:  String,
    pub country_ru: String,
    pub country_en: String,
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

////////
#[derive(Serialize)]
pub struct ListUserCommunitiesJson {
    pub name: String,
}

//////// FeaturedUserCommunities, NewsUserCommunities, NotifyUserCommunities
#[derive(Serialize)]
pub struct UniversalUserCommunityKey {
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
pub struct DesignSettings {
    pub background: String,
}

////////
#[derive(Serialize)]
pub struct Private {
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
pub struct UserProfileNotification {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
    pub message:              bool,
}
