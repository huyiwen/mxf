use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ono: u32,
    pub hno: u32,
    pub hlandlore: u32,
    pub htenant: u32,
    pub odate: String,
    pub otype: OrderType,
    pub ostart: String,
    pub oend: String,
    pub ostatus: u32,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, PartialOrd, Ord,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum OrderType {
    #[sea_orm(num_value = 0)]
    LeaseRequest,
    #[sea_orm(num_value = 1)]
    LeaseConfirm,
    #[sea_orm(num_value = 2)]
    CancelRequest,
    #[sea_orm(num_value = 3)]
    CancelConfirm,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
