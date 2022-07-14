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
pub fn get_featured_friends_json(&self, page: i32) -> Json<UniversalUserCommunityKeysJson> {
    let mut next_page_number = 0;
    let keys: Vec<UniversalUserCommunityKeyJson>;
    let count = self.get_featured_friends_count();

    if page > 1 {
        let step = (page - 1) * 20;
        keys = self.get_featured_friends(20, step.into());
        if count > (page * 20).try_into().unwrap() {
            next_page_number = page + 1;
        }
    }
    else {
        keys = self.get_featured_friends(20, 0);
        if count > 20.try_into().unwrap() {
            next_page_number = 2;
        }
    }
    return Json(UniversalUserCommunityKeysJson {
        keys: keys,
        next_page: next_page_number,
    });
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
