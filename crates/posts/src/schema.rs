table! {
    community_post_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        post_list_id -> Int4,
    }
}

table! {
    community_post_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    community_post_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

table! {
    post_comment_reactions (id) {
        id -> Int4,
        post_comment_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    post_comments (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        user_name -> Varchar,
        user_link -> Varchar,
        user_image -> Nullable<Varchar>,
        sticker_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
        types -> Char,
        created -> Timestamp,
        repost -> Int4,
        reactions -> Int4,
    }
}

table! {
    post_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        post_list_id -> Int4,
        can_see_item -> Nullable<Char>,
        can_see_comment -> Nullable<Char>,
        create_item -> Nullable<Char>,
        create_comment -> Nullable<Char>,
        can_copy -> Nullable<Char>,
    }
}

table! {
    post_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        owner_name -> Varchar,
        owner_link -> Varchar,
        owner_image -> Nullable<Varchar>,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        can_see_comment -> Char,
        create_el -> Char,
        create_comment -> Char,
        copy_el -> Char,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    post_reactions (id) {
        id -> Int4,
        post_id -> Int4,
        field_1 -> Int4,
        field_2 -> Int4,
        field_3 -> Int4,
        field_4 -> Int4,
        field_5 -> Int4,
        field_6 -> Int4,
        field_7 -> Int4,
        field_8 -> Int4,
        field_9 -> Int4,
        field_10 -> Int4,
        field_11 -> Int4,
        field_12 -> Int4,
        field_13 -> Int4,
        field_14 -> Int4,
        field_15 -> Int4,
        field_16 -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        content -> Nullable<Varchar>,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        owner_name -> Varchar,
        owner_link -> Varchar,
        owner_image -> Nullable<Varchar>,
        post_list_id -> Int4,
        types -> Char,
        attach -> Nullable<Varchar>,
        comment_enabled -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        is_signature -> Bool,
        parent_id -> Nullable<Int4>,
        reactions -> Int4,
    }
}

table! {
    posts_perms (id) {
        id -> Int4,
        user_id -> Int4,
        can_see_post -> Nullable<Char>,
        can_see_post_comment -> Nullable<Char>,
        can_copy_post -> Nullable<Char>,
        can_work_post -> Nullable<Char>,
    }
}

table! {
    user_post_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        post_list_id -> Int4,
    }
}

table! {
    user_post_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Char,
    }
}

table! {
    user_post_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
    }
}

joinable!(post_comments -> posts (post_id));
joinable!(posts -> post_lists (post_list_id));

allow_tables_to_appear_in_same_query!(
    community_post_list_collections,
    community_post_list_positions,
    community_post_notifications,
    post_comment_reactions,
    post_comments,
    post_list_perms,
    post_lists,
    post_reactions,
    posts,
    posts_perms,
    user_post_list_collections,
    user_post_list_positions,
    user_post_notifications,
);
