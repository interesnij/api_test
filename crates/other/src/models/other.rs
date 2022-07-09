use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    custom_links,
    sticker_categories,
    stickers,
    smile_categories,
    smiles,
    reactions,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use actix_web::web::Json;

/////// CustomLink //////
#[derive(Debug, Queryable, Serialize)]
pub struct CustomLink {
    pub id:    i32,
    pub link:  String,
    pub owner: i16,
}
#[derive(Deserialize, Insertable)]
#[table_name="custom_links"]
pub struct NewCustomLink {
    pub link:  String,
    pub owner: i16,
}

/////// StickerCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct StickerCategorie {
    pub id:           i32,
    pub name:         String,
    pub position:     i16,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
    pub description:  Option<String>,
    pub avatar:       Option<String>,
}

impl StickerCategorie {
    pub fn create_category(name: String, position: i16,
        user_id: i32, community_id: Option<i32>,
        owner_name: String, owner_link: String,
        owner_image: Option<String>, description: Option<String>,
        avatar: Option<String>) -> StickerCategorie {
        let _connection = establish_connection();
        let new_form = NewStickerCategorie {
            name:         name,
            position:     position,
            user_id:      user_id,
            community_id: community_id,
            owner_name:   owner_name,
            owner_link:   owner_link,
            owner_image:  owner_image,
            description:  description,
            avatar:       avatar,
        };
        let new_cat = diesel::insert_into(schema::sticker_categories::table)
            .values(&new_form)
            .get_result::<StickerCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i16,
        owner_name: String, owner_link: String,
        owner_image: Option<String>, description: Option<String>,
        avatar: Option<String>) -> &StickerCategorie {
        let _connection = establish_connection();
        let new_form = NewStickerCategorie {
            name:         name,
            position:     position,
            user_id:      self.user_id,
            community_id: self.community_id,
            owner_name:   owner_name,
            owner_link:   owner_link,
            owner_image:  owner_image,
            description:  description,
            avatar:       avatar,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<StickerCategorie>(&_connection)
            .expect("Error.");
        return self;
    }
    pub fn get_image(&self) -> &str {
        if self.avatar.is_some() {
            return self.avatar.as_deref().unwrap();
        }
        else {
            return "/static/images/no_img/smile.gif";
        }
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="sticker_categories"]
pub struct NewStickerCategorie {
    pub name:         String,
    pub position:     i16,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub owner_name:   String,
    pub owner_link:   String,
    pub owner_image:  Option<String>,
    pub description:  Option<String>,
    pub avatar:       Option<String>,
}

/////// Stickers //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct Sticker {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

impl Sticker {
    pub fn create_sticker(name: String, position: i16,
        category_id: i32, image: String) -> Sticker {
        let _connection = establish_connection();
        let new_form = NewSticker {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        let new_sticker = diesel::insert_into(schema::stickers::table)
            .values(&new_form)
            .get_result::<Sticker>(&_connection)
            .expect("Error.");
        return new_sticker;
    }
    pub fn edit_sticker(&self, name: String, position: i16,
        category_id: i32, image: String) -> &Sticker {
        let _connection = establish_connection();
        let new_form = NewSticker {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Sticker>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="stickers"]
pub struct NewSticker {
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

/////// SmileCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct SmileCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
}

impl SmileCategorie {
    pub fn get_smiles(&self) -> Vec<Smile> {
        use crate::schema::smiles::dsl::smiles;
        let _connection = establish_connection();

        return smiles
            .filter(schema::smiles::smile_categorie_id.eq(self.id))
            .order(schema::smiles::position.asc())
            .load::<Smile>(&_connection)
            .expect("E.");
    }
    pub fn create_category(name: String, position: i16,
        description: Option<String>) -> SmileCategorie {
        let _connection = establish_connection();
        let new_form = NewSmileCategorie {
            name:        name,
            position:    position,
            description: description,
        };
        let new_cat = diesel::insert_into(schema::smile_categories::table)
            .values(&new_form)
            .get_result::<SmileCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i16,
        description: Option<String>) -> &SmileCategorie {
        let _connection = establish_connection();
        let new_form = NewSmileCategorie {
            name:        name,
            position:    position,
            description: description,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<SmileCategorie>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="smile_categories"]
pub struct NewSmileCategorie {
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
}

/////// Smiles //////
#[derive(Queryable, Serialize, Deserialize)]
pub struct Smile {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

impl Smile {
    pub fn create_smile(name: String, position: i16,
        category_id: i32, image: String) -> Smile {
        let _connection = establish_connection();
        let new_form = NewSmile {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        let new_smile = diesel::insert_into(schema::smiles::table)
            .values(&new_form)
            .get_result::<Smile>(&_connection)
            .expect("Error.");
        return new_smile;
    }
    pub fn edit_smile(&self, name: String, position: i16,
        category_id: i32, image: String) -> &Smile {
        let _connection = establish_connection();
        let new_form = NewSmile {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Smile>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="smiles"]
pub struct NewSmile {
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

/////// Reactions //////

///// Типы реакций
    // 1 thumbs_up     палец вверх
    // 2 thumbs_down   палец вниз
    // 3 red_heart     красное сердце
    // 4 fire          огонь
    // 5 love_face     лицо с поцелуями
    // 6 clapping      апплодисменты
    // 7 beaming       смеющееся лицо
    // 8 thinking      размышляющее лицо
    // 9 exploding     взрывающийся мозг
    // 10 screaming    ужасающееся лицо
    // 11 evil         очень злое лицо
    // 12 crying       плачущее лицо
    // 13 party        вечеринка
    // 14 star_face    звезды в глазах
    // 15 vomiting     рвота на лице
    // 16 pile_of_poo  куча какашек

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Reaction {
    pub id:        i32,
    pub types:     i16,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}

impl Reaction {
    pub fn create_reaction(types: i16, image: String, gif: String,
        name: String, is_active: bool, position: i16) -> Reaction {
        let _connection = establish_connection();
        let new_form = NewReaction {
            types:     types,
            image:     image,
            gif:       gif,
            name:      name,
            is_active: is_active,
            position:  position,
        };
        let new_reaction = diesel::insert_into(schema::reactions::table)
            .values(&new_form)
            .get_result::<Reaction>(&_connection)
            .expect("Error.");
        return new_reaction;
    }
    pub fn edit_reaction(&self, types: i16, image: String, gif: String,
        name: String, is_active: bool, position: i16) -> &Reaction {
        let _connection = establish_connection();
        let new_form = NewReaction {
            types:     types,
            image:     image,
            gif:       gif,
            name:      name,
            is_active: is_active,
            position:  position,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Reaction>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="reactions"]
pub struct NewReaction {
    pub types:     i16,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}
