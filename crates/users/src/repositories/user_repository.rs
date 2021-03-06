use bcrypt::{hash, verify, DEFAULT_COST};
use rbatis::crud::CRUD;
use rbatis::{core::Result, rbatis::Rbatis, snowflake::Snowflake};

use crate::models::{User, UserSignup};

pub async fn create(user_data: &UserSignup, rb: &Rbatis, sflake: &Snowflake) -> Option<User> {
    log::info!("Create user");
    let users_ids: Vec<User> = rb.fetch_list_by_column("id",&["1".to_string()]).await.unwrap();
    let count = users_ids.len() + 1;
    let link = "/id".to_string() + &count.to_string() + &"/".to_string();

    let user = User {
        id: sflake.generate().unsigned_abs(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        phone: user_data.phone.clone(),
        types: 1,
        gender: user_data.gender.clone(),
        device: user_data.device.clone(),
        language: user_data.language.clone(),
        perm: 1,
        level: 100,
        password: hash(user_data.password.clone(), 8).unwrap(),
        link: link,
        city: None,
        status: None,
        b_avatar: None,
        s_avatar: None,
        email: None,
        birthday: *rbatis::DateNative::now(),
        last_activity: *rbatis::DateTimeNative::now(),
    };

    match rb.save(&user, &[]).await{
        Ok(res) =>
        {
            log::info!("Successfully create user {}", user.last_name.clone());
            Some(user)
        },
        Err(err) => {
            log::error!("Failed create user: {}", err.to_string());
            None
        }
    }
}

pub async fn update() -> Result<User> {
    todo!()
}

pub async fn find_by_id(id: u64) -> Result<User> {
    todo!()
}

pub async fn find_by_phone(phone: &String, rb: &Rbatis) -> Option<User> {
    let res: Result<User> = rb.fetch_by_column("phone", phone).await;
    match res{
        Ok(user) => Some(user),
        Err(err) => {
            log::error!("Failed find by phone: {}", err.to_string());
            None
        }
    }
}
