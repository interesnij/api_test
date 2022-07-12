use diesel::prelude::*;
use crate::schema;

use serde::{Serialize, Deserialize};
use crate::models::{
    Reaction, CustomLink,
    SmileCategorie, Smile,
    StickerCategorie, Sticker,
};

#[derive(Serialize)]
// это реакция
pub struct ReactionJson {
    pub types:     i16,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}

#[derive(Serialize)]
// это ссылка пользователя или сообщества
pub struct CustomLinksJson {
    pub links:     Vec<CustomLinkJson>,
    pub next_page: i32,
}
#[derive(Serialize)]
// это ссылка пользователя или сообщества
pub struct CustomLinkJson {
    pub link:  String,
    pub owner: i16,
}

#[derive(Serialize)]
// это карточки карегорий стикеров
pub struct StickerCategoriesJson {
    pub categories:  Vec<CardStickerCategoryJson>,
    pub next_page:   i32,
}
#[derive(Serialize)]
// это карточка карегории стикеров
pub struct CardStickerCategoryJson {
    pub id:     i32,
    pub avatar: Option<String>,
}

#[derive(Serialize)]
// это страница карегория стикеров
pub struct StickerCategorieDetailJson {
    pub name:        String,
    pub owner_name:  String,
    pub owner_link:  String,
    pub owner_image: Option<String>,
    pub description: Option<String>,
    pub avatar:      Option<String>,
    pub stickers:    Vec<CardStickerJson>,
    pub next_page:   i32,
}
#[derive(Serialize)]
// это стикер
pub struct CardStickerJson {
    pub name:        String,
    pub image:       String,
}

#[derive(Serialize)]
// это карточки карегорий смайлов
pub struct SmileCategories {
    pub categories:  Vec<CardSmileCategoryJson>,
    pub next_page:   i32,
}
#[derive(Serialize)]
// это карточка карегории смайлов
pub struct CardSmileCategoryJson {
    pub name: Option<String>,
}

#[derive(Serialize)]
// это страница карегория смайлов
pub struct SmileCategorieDetail {
    pub name:      String,
    pub next_page: i32,
}
#[derive(Serialize)]
// это смайл
pub struct CardSmile {
    pub name:  String,
    pub image: String,
}


pub fn get_reaction(pk: i32) -> Reaction {
    use crate::schema::reactions::dsl::reactions;
    let _connection = establish_connection();
    return reactions
        .filter(schema::reactions::id.eq(pk))
        .load::<Reaction>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_custom_link(link: String) -> CustomLink {
    use crate::schema::custom_links::dsl::custom_links;
    let _connection = establish_connection();
    return custom_links
        .filter(schema::custom_links::link.eq(link))
        .load::<CustomLink>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_sticker_category(id: i32) -> StickerCategorie {
    use crate::schema::sticker_categories::dsl::sticker_categories;
    let _connection = establish_connection();
    return sticker_categories
        .filter(schema::sticker_categories::id.eq(id))
        .load::<StickerCategorie>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_sticker(id: i32) -> Sticker {
    use crate::schema::stickers::dsl::stickers;
    let _connection = establish_connection();
    return stickers
        .filter(schema::stickers::id.eq(id))
        .load::<Sticker>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}

pub fn get_smile_category(id: i32) -> SmileCategorie {
    use crate::schema::smile_categories::dsl::smile_categories;
    let _connection = establish_connection();
    return smile_categories
        .filter(schema::smile_categories::id.eq(id))
        .load::<SmileCategorie>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}
pub fn get_smile(id: i32) -> Smile {
    use crate::schema::smiles::dsl::smiles;
    let _connection = establish_connection();
    return smiles
        .filter(schema::smiles::id.eq(id))
        .load::<Smile>(&_connection)
        .expect("E.")
        .into_iter()
        .nth(0)
        .unwrap();
}



pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_count_for_ru(count: i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}
pub fn get_count_for_ru_alt(count: i32, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    if a == 1 && b != 11 {
        return word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return word2;
    }
    else {
        return word3;
    }
}
