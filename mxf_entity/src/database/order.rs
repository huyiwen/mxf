use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ono: u32,
    pub hno: u32,
    pub hlandlore: u32,
    pub htenant: u32,
    pub odate: NaiveDateTime,
    pub otype: OrderType,
    pub ostart: NaiveDateTime,
    pub oend: NaiveDateTime,
    pub ostatus: u32,
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
pub enum OrderType {
    #[sea_orm(num_value = 0)]
    Deleted,
    #[sea_orm(num_value = 1)]
    LeaseRequest,
    #[sea_orm(num_value = 2)]
    LeaseConfirm,
    #[sea_orm(num_value = 3)]
    CancelRequest,
    #[sea_orm(num_value = 4)]
    CancelConfirm,
}

impl Model {
    pub fn is_confirmed(&self) -> bool {
        self.otype == OrderType::LeaseConfirm || self.otype == OrderType::CancelConfirm
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::house_listing::Entity",
        from = "Column::Hno",
        to = "super::house_listing::Column::Hno"
    )]
    HouseListing,
}

impl Related<super::house_listing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HouseListing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
