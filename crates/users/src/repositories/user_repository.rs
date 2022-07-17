use bcrypt::{hash, verify, DEFAULT_COST};
use rbatis::crud::CRUD;
use rbatis::{core::Result, rbatis::Rbatis, snowflake::Snowflake};

use crate::models::{User, UserSignup};

pub async fn create(user_data: &UserSignup, rb: &Rbatis, sflake: &Snowflake) -> Option<User> {
    log::info!("Create user");
    let user = User {
        id: sflake.generate().unsigned_abs(),
        username: user_data.username.clone(),
        phone: user_data.phone.clone(),
        created_date: rbatis::DateTimeNative::now(),
        password: hash(user_data.password.clone(), 8).unwrap(),
    };

    match rb.save(&user, &[]).await{
        Ok(res) =>
        {
            log::info!("Successfully create user {}", user.username.clone());
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
