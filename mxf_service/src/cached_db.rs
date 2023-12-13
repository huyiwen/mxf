use mini_moka::sync::Cache;
use sea_orm::*;
use std::time::Duration;

use mxf_entity::{HouseFilter, HouseListingEntity, HouseListingModel};

pub struct CachedDb {
    num_pages_cache: Cache<String, u64>,
}

impl CachedDb {
    pub fn init() -> Self {
        let num_pages_cache = Cache::builder()
            .time_to_live(Duration::from_secs(30 * 60))
            .build();
        return CachedDb { num_pages_cache };
    }

    pub async fn find_house_by_id(
        &self,
        db: &DbConn,
        hno: u8,
    ) -> Result<Option<HouseListingModel>, DbErr> {
        HouseListingEntity::find_by_id(hno).one(db).await
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_houses_in_page(
        &self,
        db: &DbConn,
        house_filter: HouseFilter<'_>,
        posts_per_page: u8,
    ) -> Result<(Vec<HouseListingModel>, u64), DbErr> {
        let filter_string = house_filter.to_string();
        let posts_per_page = posts_per_page as u64;
        let paginator = HouseListingEntity::find()
            .filter(Condition::from(house_filter))
            .paginate(db, posts_per_page);
        let do_insert = !self.num_pages_cache.contains_key::<String>(&filter_string);
        if do_insert {
            self.num_pages_cache
                .insert(filter_string.clone(), paginator.num_pages().await?);
        }
        let num_pages = self.num_pages_cache.get::<String>(&filter_string).unwrap();
        // let num_pages = paginator.num_pages().await?;
        paginator
            .fetch_page(house_filter.page)
            .await
            .map(|p| (p, num_pages))
    }
}
