use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, PartialOrd, Ord,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uno: u32,
    pub uname: String,
    pub(crate) upass: String,
    pub utype: UserType,
    pub uemail: String,
    pub uphone: String,
}

impl Model {
    pub fn default() -> Self {
        Self {
            uno: 0,
            uname: "".to_string(),
            upass: "".to_string(),
            utype: UserType::User,
            uemail: "".to_string(),
            uphone: "".to_string(),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, PartialOrd, Ord,
)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum UserType {
    #[sea_orm(num_value = 0)]
    Deleted,
    #[sea_orm(num_value = 1)]
    Admin,
    #[sea_orm(num_value = 2)]
    Employee,
    #[sea_orm(num_value = 3)]
    User,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::house_listing::Entity")]
    HouseListing,
}

impl Related<super::house_listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HouseListing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
