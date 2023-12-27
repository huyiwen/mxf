use mxf_service::pool::SeaOrmPool;
use sea_orm_rocket::Database;

#[derive(Database, Debug)]
#[database("mxf")]
pub struct MXFDb(SeaOrmPool);
