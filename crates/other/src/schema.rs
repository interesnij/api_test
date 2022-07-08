table! {
    custom_links (id) {
        id -> Int4,
        link -> Varchar,
        owner -> Int2,
    }
}

table! {
    folder_items (id) {
        id -> Int4,
        folder_id -> Int4,
        types -> Int2,
        object_id -> Int4,
        position -> Int2,
    }
}

table! {
    folders (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        owner_name -> Varchar,
        owner_link -> Varchar,
        owner_image -> Nullable<Varchar>,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        can_see_el -> Char,
        create_el -> Char,
        copy_el -> Char,
    }
}

table! {
    smile_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        description -> Nullable<Varchar>,
    }
}

table! {
    smiles (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        category_id -> Int4,
        image -> Varchar,
    }
}

table! {
    sticker_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        owner_name -> Varchar,
        owner_link -> Varchar,
        owner_image -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
    }
}

table! {
    stickers (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        category_id -> Int4,
        image -> Varchar,
    }
}

joinable!(smiles -> smile_categories (category_id));
joinable!(stickers -> sticker_categories (category_id));

allow_tables_to_appear_in_same_query!(
    custom_links,
    folder_items,
    folders,
    smile_categories,
    smiles,
    sticker_categories,
    stickers,
);
