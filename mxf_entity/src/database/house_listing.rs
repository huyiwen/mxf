use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "house_listings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub hno: u32,
    pub hdistrict: String,
    pub haddr: String,
    pub hlo: String,
    pub hflr: u32,
    pub harea: u32,
    pub hprice: u32,
    pub hlandlore: u32,
    pub hsuite: String,
    pub hunlisted: ListStatus,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    Copy,
)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum ListStatus {
    #[sea_orm(num_value = 0)]
    Listed,
    #[sea_orm(num_value = 1)]
    Unlisted,
}

impl Default for ListStatus {
    fn default() -> Self {
        ListStatus::Listed
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Hlandlore",
        to = "super::user::Column::Uno"
    )]
    User,
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
