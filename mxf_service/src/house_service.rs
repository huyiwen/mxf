use mini_moka::sync::Cache;
use sea_orm::*;
use std::time::Duration;

use mxf_entity::{HouseFilter, HouseListingEntity, HouseListingModel, MXFError};

pub struct HouseService {
    num_pages_cache: Cache<String, u64>,
}

impl HouseService {
    pub fn init() -> Self {
        let num_pages_cache = Cache::builder()
            .time_to_live(Duration::from_secs(30 * 60))
            .build();
        return HouseService { num_pages_cache };
    }

    pub async fn get_house_by_hno(
        &self,
        db: &DbConn,
        hno: u32,
    ) -> Result<HouseListingModel, MXFError> {
        HouseListingEntity::find_by_id(hno)
            .one(db)
            .await?
            .ok_or(MXFError::UnknownError(format!(
                "House with hno {} not found",
                hno
            )))
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_houses_in_page(
        &self,
        db: &DbConn,
        house_filter: HouseFilter<'_>,
        posts_per_page: u8,
    ) -> Result<(Vec<HouseListingModel>, u64), MXFError> {
        let filter_string = house_filter.to_string();
        let posts_per_page = posts_per_page as u64;
        println!(
            "filter {}: {:?} {:?}",
            filter_string,
            house_filter,
            Condition::from(house_filter)
        );
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
        Ok((
            paginator.fetch_page(house_filter.page - 1).await?,
            num_pages,
        ))
    }
}
