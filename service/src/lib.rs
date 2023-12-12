#[macro_use]
extern crate rocket;

pub(self) mod house_listing;
pub mod mutation;
pub mod pool;
pub mod query;

pub use pool::Db;
pub use query::Query;
