table! {
    communities_memberships (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        is_administrator -> Bool,
        is_moderator -> Bool,
        is_editor -> Bool,
        is_advertiser -> Bool,
        created -> Timestamp,
        visited -> Int4,
    }
}

table! {
    community_banner_users (id) {
        id -> Int4,
        community_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    community_categorys (id) {
        id -> Int4,
        name -> Varchar,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    community_infos (id) {
        id -> Int4,
        community_id -> Int4,
        posts -> Int4,
        members -> Int4,
        photos -> Int4,
        goods -> Int4,
        tracks -> Int4,
        videos -> Int4,
        docs -> Int4,
        articles -> Int4,
        survey -> Int4,
        planners -> Int4,
        avatar_id -> Nullable<Int4>,
    }
}

table! {
    community_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        community_invite -> Bool,
    }
}

table! {
    community_privates (id) {
        id -> Int4,
        community_id -> Int4,
        can_see_member -> Char,
        can_see_info -> Char,
        can_send_message -> Char,
        can_see_post -> Char,
        can_see_photo -> Char,
        can_see_good -> Char,
        can_see_video -> Char,
        can_see_music -> Char,
        can_see_planner -> Char,
        can_see_doc -> Char,
        can_see_survey -> Char,
        can_see_settings -> Char,
        can_see_log -> Char,
        can_see_stat -> Char,
        can_see_forum -> Char,
    }
}

table! {
    community_subcategorys (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    communitys (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        types -> Int2,
        perm -> Char,
        level -> Int2,
        link -> Varchar,
        b_avatar -> Nullable<Varchar>,
        s_avatar -> Nullable<Varchar>,
        cover -> Nullable<Varchar>,
        category_id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    communities_memberships,
    community_banner_users,
    community_categorys,
    community_infos,
    community_notifications,
    community_privates,
    community_subcategorys,
    communitys,
);
