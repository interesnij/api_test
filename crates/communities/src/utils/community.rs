use serde::Serialize;


////////
#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct UsersJson {
    pub users:     Vec<CardOwnerJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
// это объект пользователя
pub struct CardUserJson {
    pub id:          i32,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
}

////////
#[derive(Serialize)]
pub struct CommunityCategoryJson {
    pub id:     i32,
    pub name:   String,
    pub avatar: Option<String>,
}

////////
#[derive(Serialize)]
pub struct CommunitySubcategoryJson {
    pub id:          i32,
    pub name:        String,
    pub avatar:      Option<String>,
}
////////
#[derive(Serialize)]
pub struct CommunityDetailJson {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub status:      Option<String>,
    pub types:       i16,
    pub perm:        i16,
    pub link:        String, // community.get_link()
    pub image:       String,
    pub cover:       Option<String>,
    pub user_id:     i32,
}

////////
#[derive(Serialize)]
// универсальный сериализатор для списков пользователей
pub struct CommunityInvitesJson {
    pub users:     Vec<CardCommunityInviteJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
// это объект пользователя
pub struct CardCommunityInviteJson {
    pub first_name: String,
    pub last_name:  String,
    pub link:       String,
    pub image:      String,
}

////////
#[derive(Serialize)]
// это объект пользователя
pub struct CommunityInfoJson {
    pub posts:     i32,
    pub members:   i32,
    pub photos:    i32,
    pub goods:     i32,
    pub tracks:    i32,
    pub videos:    i32,
    pub docs:      i32,
    pub articles:  i32,
    pub survey:    i32,
    pub planners:  i32,
    pub avatar_id: Option<i32>,
}

////////
#[derive(Serialize)]
pub struct UserPrivateJson {
    pub can_see_member:    String,
    pub can_see_info:      String,
    pub can_see_friend:    String,
    pub can_send_message:  String,
    pub can_see_post:      String,
    pub can_see_photo:     String,
    pub can_see_good:      String,
    pub can_see_video:     String,
    pub can_see_music:     String,
    pub can_see_planner:   String,
    pub can_see_doc:       String,
    pub can_see_survey:    String,
    pub can_see_settings:  String,
    pub can_see_log:       String,
    pub can_see_stat:      String,
    pub can_see_forum:     String,
}

////////
#[derive(Serialize)]
// это объект пользователя
pub struct CommunityNotificationJson {
    pub connection_request:   bool,
    pub connection_confirmed: bool,
    pub community_invite:     bool,
}

////////
#[derive(Serialize)]
pub struct CommunityVisiblePermJson {
    pub can_see_info:            String,
    pub can_see_community:       String,
    pub can_see_member:          String,
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
    pub can_see_forum:           String,
    pub can_see_forum_comment:   String,
}

////////
#[derive(Serialize)]
pub struct CommunityWorkPermJson {
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
