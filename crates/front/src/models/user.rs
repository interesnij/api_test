use serde::{Deserialize, Serialize};
use validator::Validate;
use wasm_bindgen::JsValue;


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct UserDetail {
    pub id:            u64,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         u16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          u16,
    pub link:          String, // community.get_link()
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub image:         String,
    pub birthday:      String,
    pub last_activity: String,
}

#[derive(Deserialize, Debug, Clone, Validate, Serialize, PartialEq)]
pub struct UserLogin {
    #[validate(length(min = 8))]
    pub phone: String,
    #[validate(length(min = 8))]
    pub password: String,
}

impl UserLogin {
    pub fn default() -> UserLogin {
        UserLogin {
            phone: String::default(),
            password: String::default(),
        }
    }

    pub fn is_empty(&self)->bool{
        !(self.phone.len() > 0 && self.password.len() > 0)
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq)]
pub struct UserSignup {
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          i16,
    pub password:      String,
    pub link:          String,
    pub birthday:      String,
}

impl UserSignup {
    pub fn default() -> UserSignup {
        UserSignup {
            first_name: String::default(),
            last_name: String::default(),
            phone: String::default(),
            gender: String::default(),
            device: String::default(),
            language: String::default(),
            perm: 0,
            password: String::default(),
            link: String::default(),
            birthday: String::default(),
        }
    }

    pub fn is_empty(&self)->bool{
        !(self.first_name.len() > 0 && self.last_name.len() > 0 && self.password.len() > 0)
    }
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct UserToken{
    pub token: String
}
