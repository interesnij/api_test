use crate::schema;
use diesel::prelude::*;
use crate::schema::{
    phone_codes,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;

/////// PhoneCode //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct PhoneCode {
    pub id:     i32,
    pub phone:  String,
    pub code:   i32,
}
#[derive(Deserialize, Insertable)]
#[table_name="phone_codes"]
pub struct NewPhoneCode {
    pub phone:  String,
    pub code:   i32,
}
