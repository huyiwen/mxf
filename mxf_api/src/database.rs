use mxf_service::pool::SeaOrmPool;
use sea_orm_rocket::Database;

#[derive(Database, Debug)]
#[database("house_listings")]
pub struct HouseDb(SeaOrmPool);

#[derive(Database, Debug)]
#[database("users")]
pub struct UserDb(SeaOrmPool);
