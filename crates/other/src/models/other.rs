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
use actix_web::web::Json;
use crate::utils::{
    establish_connection,
    ReactionJson, CustomLinkJson, CustomLinksJson,
    StickerCategoriesJson, CardStickerCategoryJson, StickerCategorieDetailJson, CardStickerJson,
    SmileCategoriesJson, CardSmileCategoryJson, SmileCategorieDetailJson, CardSmileJson,
};

/////// CustomLink //////
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct CustomLink {
    pub id:    i32,
    pub link:  String,
    pub owner: i16,
}

impl CustomLink {
    pub fn get_links_json(page: i32) -> Json<CustomLinksJson> {
        let mut next_page_number = 0;
        let count = CustomLink::count_links();
        let links: Vec<CustomLink>;

        if page > 1 {
            let step = (page - 1) * 20;
            links = CustomLink::get_links(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            links = CustomLink::get_links(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        let mut links_json = Vec::new();
        for i in links.iter() {
            links_json.push (
                CustomLinkJson {
                    link:  i.link.clone(),
                    owner: i.owner,
                }
            );
        }

        let data = CustomLinksJson {
            links:     links_json,
            next_page: next_page_number,
        };
        return Json(data);
    }

    pub fn get_link_json (&self) -> CustomLinkJson {
        let card = CustomLinkJson {
            link:  self.link.clone(),
            owner: self.owner,
        };
        return card;
    }
    pub fn get_links(limit: i64, offset: i64) -> Vec<CustomLink> {
        use crate::schema::custom_links::dsl::custom_links;

        let _connection = establish_connection();

        return custom_links
            .limit(limit)
            .offset(offset)
            .load::<CustomLink>(&_connection)
            .expect("E.");
    }
    pub fn count_links() -> usize {
        use crate::schema::custom_links::dsl::custom_links;

        let _connection = establish_connection();
        return custom_links
            .select(schema::custom_links::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}
#[derive(Deserialize, Insertable)]
#[table_name="custom_links"]
pub struct NewCustomLink {
    pub link:  String,
    pub owner: i16,
}

/////// StickerCategories //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
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
    pub fn create_category (
        name: String,
        position: i16,
        user_id: i32,
        community_id: Option<i32>,
        owner_name: String,
        owner_link: String,
        owner_image: Option<String>,
        description: Option<String>,
        avatar: Option<String>
    ) -> StickerCategorie {
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
    pub fn get_categories_json(page: i32) -> Json<StickerCategoriesJson> {
        let mut next_page_number = 0;
        let count = StickerCategorie::count_categories();
        let categories: Vec<StickerCategorie>;

        if page > 1 {
            let step = (page - 1) * 20;
            categories = StickerCategorie::get_categories(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            categories = StickerCategorie::get_categories(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        let mut categories_json = Vec::new();
        for i in categories.iter() {
            categories_json.push (
                CardStickerCategoryJson {
                    id:     i.id.clone(),
                    avatar: i.avatar.clone(),
                }
            );
        }

        let data = StickerCategoriesJson {
            categories: categories_json,
            next_page:  next_page_number,
        };
        return Json(data);
    }
    pub fn get_categorie_detail_json(&self, page: i32) -> Json<StickerCategorieDetailJson> {
        let mut next_page_number = 0;
        let count = self.count_stickers();
        let stickers: Vec<Sticker>;

        if page > 1 {
            let step = (page - 1) * 20;
            stickers = self.get_stickers(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            stickers = self.get_stickers(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        let mut stickers_json = Vec::new();
        for i in stickers.iter() {
            stickers_json.push (
                CardStickerJson {
                    name:  i.name.clone(),
                    image: i.image.clone(),
                }
            );
        }

        let data = StickerCategorieDetailJson {
            name:        self.name.clone(),
            owner_name:  self.owner_name.clone(),
            owner_link:  self.owner_link.clone(),
            owner_image: self.owner_image.clone(),
            description: self.description.clone(),
            avatar:      self.avatar.clone(),
            stickers:    stickers_json,
            next_page:   next_page_number,
        };
        return Json(data);
    }

    pub fn get_category_json (&self) -> CardStickerCategoryJson {
        let card = CardStickerCategoryJson {
            id:     self.id.clone(),
            avatar: self.avatar.clone(),
        };
        return card;
    }
    pub fn get_categories(limit: i64, offset: i64) -> Vec<StickerCategorie> {
        use crate::schema::sticker_categories::dsl::sticker_categories;

        let _connection = establish_connection();

        return sticker_categories
            .limit(limit)
            .offset(offset)
            .load::<StickerCategorie>(&_connection)
            .expect("E.");
    }
    pub fn get_stickers(&self, limit: i64, offset: i64) -> Vec<Sticker> {
        use crate::schema::stickers::dsl::stickers;

        let _connection = establish_connection();

        return stickers
            .limit(limit)
            .offset(offset)
            .load::<Sticker>(&_connection)
            .expect("E.");
    }
    pub fn count_stickers(&self) -> usize {
        use crate::schema::stickers::dsl::stickers;

        let _connection = establish_connection();
        return stickers
            .select(schema::stickers::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
    pub fn count_categories() -> usize {
        use crate::schema::sticker_categories::dsl::sticker_categories;

        let _connection = establish_connection();
        return sticker_categories
            .select(schema::sticker_categories::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
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
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
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
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
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
            .filter(schema::smiles::category_id.eq(self.id))
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
    pub fn get_categories_json(page: i32) -> Json<SmileCategoriesJson> {
        let mut next_page_number = 0;
        let count = SmileCategorie::count_categories();
        let categories: Vec<SmileCategorie>;

        if page > 1 {
            let step = (page - 1) * 20;
            categories = SmileCategorie::get_categories(20, step.into());
            if count > (page * 20).try_into().unwrap() {
                next_page_number = page + 1;
            }
        }
        else {
            categories = SmileCategorie::get_categories(20, 0);
            if count > 20.try_into().unwrap() {
                next_page_number = 2;
            }
        }

        let mut categories_json = Vec::new();
        for i in categories.iter() {
            categories_json.push (
                CardSmileCategoryJson {
                    name: i.name.clone(),
                }
            );
        }

        let data = SmileCategoriesJson {
            categories: categories_json,
            next_page:  next_page_number,
        };
        return Json(data);
    }

    pub fn get_category_json (&self) -> CardSmileCategoryJson {
        let card = CardSmileCategoryJson {
            name: self.name.clone(),
        };
        return card;
    }
    pub fn get_categories(limit: i64, offset: i64) -> Vec<SmileCategorie> {
        use crate::schema::smile_categories::dsl::smile_categories;

        let _connection = establish_connection();

        return smile_categories
            .limit(limit)
            .offset(offset)
            .load::<SmileCategorie>(&_connection)
            .expect("E.");
    }
    pub fn count_categories() -> usize {
        use crate::schema::smile_categories::dsl::smile_categories;

        let _connection = establish_connection();
        return smile_categories
            .select(schema::smile_categories::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_categorie_detail_json(&self) -> Json<SmileCategorieDetailJson> {

        let smiles = self.get_smiles();

        let mut smiles_json = Vec::new();
        for i in smiles.iter() {
            smiles_json.push (
                CardSmileJson {
                    name:  i.name.clone(),
                    image: i.image.clone(),
                }
            );
        }

        let data = SmileCategorieDetailJson {
            name:   self.name.clone(),
            smiles: smiles_json,
        };
        return Json(data);
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
#[derive(Queryable, Serialize, Deserialize, Identifiable)]
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

///// ???????? ??????????????
    // 1 thumbs_up     ?????????? ??????????
    // 2 thumbs_down   ?????????? ????????
    // 3 red_heart     ?????????????? ????????????
    // 4 fire          ??????????
    // 5 love_face     ???????? ?? ??????????????????
    // 6 clapping      ??????????????????????????
    // 7 beaming       ?????????????????? ????????
    // 8 thinking      ???????????????????????? ????????
    // 9 exploding     ???????????????????????? ????????
    // 10 screaming    ?????????????????????? ????????
    // 11 evil         ?????????? ???????? ????????
    // 12 crying       ???????????????? ????????
    // 13 party        ??????????????????
    // 14 star_face    ???????????? ?? ????????????
    // 15 vomiting     ?????????? ???? ????????
    // 16 pile_of_poo  ???????? ??????????????

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
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
