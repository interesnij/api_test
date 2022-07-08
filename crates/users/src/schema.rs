table! {
    design_settings (id) {
        id -> Int4,
        user_id -> Int4,
        background -> Char,
    }
}

table! {
    featured_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    follows (id) {
        id -> Int4,
        user_id -> Int4,
        followed_user -> Int4,
        view -> Bool,
        visited -> Int4,
    }
}

table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        target_user_id -> Int4,
        visited -> Int4,
    }
}

table! {
    ip_users (id) {
        id -> Int4,
        user_id -> Int4,
        ip -> Varchar,
    }
}

table! {
    list_user_communities_keys (id) {
        id -> Int4,
        types -> Char,
        name -> Varchar,
        owner -> Int4,
    }
}

table! {
    news_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    notify_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    user_anketas (id) {
        id -> Int4,
        user_id -> Int4,
        political_preferences -> Nullable<Varchar>,
        worldview -> Nullable<Varchar>,
        mainthing_in_life -> Nullable<Varchar>,
        mainthing_in_people -> Nullable<Varchar>,
        attitude_to_smoking -> Nullable<Varchar>,
        attitude_to_alcohol -> Nullable<Varchar>,
        inspiration -> Nullable<Varchar>,
    }
}

table! {
    user_blocks (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_brother_sisters (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_children_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_colleagues_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_dad_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_delete_anketas (id) {
        id -> Int4,
        user_id -> Int4,
        answer -> Char,
        other -> Nullable<Varchar>,
        created -> Timestamp,
    }
}

table! {
    user_grandsons_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_locations (id) {
        id -> Int4,
        user_id -> Int4,
        city_ru -> Nullable<Varchar>,
        city_en -> Nullable<Varchar>,
        region_ru -> Nullable<Varchar>,
        region_en -> Nullable<Varchar>,
        country_ru -> Nullable<Varchar>,
        country_en -> Nullable<Varchar>,
    }
}

table! {
    user_love_statuss (id) {
        id -> Int4,
        user_id -> Int4,
        male_status -> Nullable<Varchar>,
        female_status -> Nullable<Varchar>,
    }
}

table! {
    user_mom_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_partner_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_populate_smiles (id) {
        id -> Int4,
        user_id -> Int4,
        smile_id -> Int4,
        count -> Int4,
    }
}

table! {
    user_populate_stickers (id) {
        id -> Int4,
        user_id -> Int4,
        sticker_id -> Int4,
        count -> Int4,
    }
}

table! {
    user_privates (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_all -> Char,
        can_see_community -> Char,
        can_see_info -> Char,
        can_see_friend -> Char,
        can_send_message -> Char,
        can_add_in_chat -> Char,
        can_see_post -> Char,
        can_see_photo -> Char,
        can_see_good -> Char,
        can_see_video -> Char,
        can_see_music -> Char,
        can_see_planner -> Char,
        can_see_doc -> Char,
        can_see_survey -> Char,
    }
}

table! {
    user_profile_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        community_invite -> Bool,
    }
}

table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Int4,
        posts -> Int4,
        friends -> Int4,
        follows -> Int4,
        communities -> Int4,
        photos -> Int4,
        goods -> Int4,
        docs -> Int4,
        tracks -> Int4,
        videos -> Int4,
        articles -> Int4,
        planners -> Int4,
        avatar_id -> Nullable<Int4>,
        survey -> Int4,
        saved_playlist -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Varchar,
        types -> Int2,
        gender -> Char,
        device -> Char,
        language -> Char,
        perm -> Int2,
        level -> Int2,
        password -> Varchar,
        link -> Varchar,
        city -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        b_avatar -> Nullable<Varchar>,
        s_avatar -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        birthday -> Date,
        last_activity -> Timestamp,
    }
}

joinable!(design_settings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    design_settings,
    featured_user_communities,
    follows,
    friends,
    ip_users,
    list_user_communities_keys,
    news_user_communities,
    notify_user_communities,
    user_anketas,
    user_blocks,
    user_brother_sisters,
    user_children_ones,
    user_colleagues_ones,
    user_dad_ones,
    user_delete_anketas,
    user_grandsons_ones,
    user_locations,
    user_love_statuss,
    user_mom_ones,
    user_partner_ones,
    user_populate_smiles,
    user_populate_stickers,
    user_privates,
    user_profile_notifications,
    user_profiles,
    users,
);
