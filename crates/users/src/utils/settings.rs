use crate::utils::{
    establish_connection,
    UserDetailJson,
    LocationsJson,
    LocationJson,
    ProfileJson,
    IpsJson,
    ListsUserCommunitiesJson,
    UniversalUserCommunityKeysJson,
    DesignSettingsJson,
    UserPrivateJson,
    UserProfileNotificationJson,
    UserPopulateStickerJson,
    UserPopulateSmileJson,
    FriendsVisiblePermJson,
    PhoneCodeJson,
    UserWorkPermJson,
    UsersListJson,
};


pub fn get_blocked_users_json(&self, page: i32) -> Json<UsersListJson> {
    let mut next_page_number = 0;
    let users: Vec<CardUserJson>;
    let count = self.count_blacklist();

    if page > 1 {
        let step = (page - 1) * 20;
        users = self.get_blocked_users(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        users = self.get_blocked_users(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }
    return Json(UsersListJson {
        description: "Черный спсок".to_string(),
        users: users,
        next_page: next_page_number,
    });
}

pub fn get_blocked_users(&self, limit: i64, offset: i64) -> Vec<CardUserJson> {
    use crate::schema::{
        user_blocks::dsl::user_blocks,
        users::dsl::users,
    };

    let _connection = establish_connection();
    let all_user_blocks = user_blocks
        .filter(schema::user_blocks::user_id.eq(self.id))
        .order(schema::user_blocks::id.desc())
        .limit(limit)
        .offset(offset)
        .select(schema::user_blocks::target_id)
        .load::<i32>(&_connection)
        .expect("E");
    blocked_users = users
        .filter(schema::users::id.eq_any(stack))
        .filter(schema::users::types.lt(10))
        .load::<User>(&_connection)
        .expect("E.");
    let mut blocked_json = Vec::new();
    for user in blocked_users {
        blocked_json.push (CardUserJson {
            id:         user.id,
            first_name: user.first_name.clone(),
            last_name:  user.last_name.clone(),
            link:       user.link.clone(),
            image:      user.s_avatar.clone(),
        });
    }
    return blocked_json;
}

pub fn get_featured_friends(&self, limit: i64, offset: i64) -> Vec<UniversalUserCommunityKeyJson> {
    use crate::schema::featured_user_communities::dsl::featured_user_communities;
    use crate::models::FeaturedUserCommunitie;

    let _connection = establish_connection();
    let featured_friends = featured_user_communities
        .filter(schema::featured_user_communities::owner.eq(self.id))
        .filter(schema::featured_user_communities::community_id.is_null())
        .order(schema::featured_user_communities::id.desc())
        .limit(limit)
        .offset(offset)
        .load::<FeaturedUserCommunitie>(&_connection)
        .expect("E.");

    let mut stack = Vec::new();
    for i in featured_friends {
        stack.push(UniversalUserCommunityKeyJson {
            id:           i.id,
            list_id:      i.list_id,
            mute:         i.mute,
            sleep:        i.sleep.unwrap().format("%d-%m-%Y в %H:%M").to_string(),
            owner_name:   i.owner_name.clone(),
            owner_link:   i.owner_link.clone(),
            owner_image:  i.owner_image.clone(),
        })
    }
    return stack;
}
pub fn get_populate_smiles_json(&self) -> Json<Vec<UserPopulateSmileJson>> {
    use crate::schema::user_populate_smiles::dsl::user_populate_smiles;

    let _connection = establish_connection();
    let all_populate_smiles = user_populate_smiles
        .filter(schema::user_populate_smiles::user_id.eq(self.id))
        .order(schema::user_populate_smiles::count.desc())
        .limit(20)
        .select((schema::user_populate_smiles::smile_id, schema::user_populate_smiles::image))
        .load::<(i32, String)>(&_connection)
        .expect("E");
    let mut smiles_json = Vec::new();
    for smile in all_populate_smiles.iter() {
        smiles_json.push(UserPopulateSmileJson {
            smile_id: smile.0,
            image:    smile.1.clone(),
        });
    }
    return Json(smiles_json);
}
