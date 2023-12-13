use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uno: u32,
    pub uname: String,
    pub upass: String,
    pub utype: UserType,
    pub uemail: String,
    pub uphone: String,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum UserType {
    #[sea_orm(num_value = 0)]
    Admin,
    #[sea_orm(num_value = 1)]
    Employee,
    #[sea_orm(num_value = 2)]
    User,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
