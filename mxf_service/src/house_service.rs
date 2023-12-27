use mini_moka::sync::Cache;
use sea_orm::*;
use std::time::Duration;

use mxf_entity::{
    HouseFilter, HouseListingColumn, HouseListingEntity, HouseListingModel, HouseListingActiveModel, MXFError, ListStatus
};

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

    pub async fn get_houses_by_landlore(
        &self,
        db: &DbConn,
        hlandlore: u32,
        listed_only: bool,
    ) -> Result<Vec<HouseListingModel>, MXFError> {
        let cond = Condition::any().add(HouseListingColumn::Hlandlore.eq(hlandlore))
            .add_option(listed_only.then_some(HouseListingColumn::Hunlisted.eq(ListStatus::Listed)));
        HouseListingEntity::find()
            .filter(cond)
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_next_hno(&self, db: &DbConn) -> Result<u32, MXFError> {
        let hno = HouseListingEntity::find()
            .order_by_desc(HouseListingColumn::Hno)
            .one(db)
            .await?
            .ok_or(MXFError::UnknownError(format!("No houses found")))?
            .hno;
        Ok(hno + 1)
    }

    /// If ok, returns (post models, num pages).
    pub async fn get_houses_in_page(
        &self,
        db: &DbConn,
        house_filter: HouseFilter<'_>,
        posts_per_page: u8,
        listed_only: bool,
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
            .filter(
                Condition::from(house_filter)
                    .add_option(listed_only.then_some(HouseListingColumn::Hunlisted.eq(ListStatus::Listed)))
            )
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

    pub async fn new_house(
        &self,
        db: &DbConn,
        house_listing: HouseListingModel,
        uno: u32,
    ) -> Result<u32, MXFError> {
        let mut house: HouseListingActiveModel = house_listing.into();
        house.hno = NotSet;
        house.hlandlore = Set(uno);
        let res = HouseListingEntity::insert(house).exec(db).await?;
        Ok(res.last_insert_id)
    }

    pub async fn verify_landlore(&self, db: &DbConn, hno: u32, uno: u32) -> Result<(), MXFError> {
        let hlandlore = self.get_house_by_hno(db, hno).await?.hlandlore;
        if hlandlore != uno {
            Err(MXFError::NotLandlore(uno))
        } else {
            Ok(())
        }
    }

    pub async fn modify_house(
        &self,
        db: &DbConn,
        house_listing: HouseListingModel,
        uno: u32,
    ) -> Result<u32, MXFError> {
        self.verify_landlore(db, house_listing.hno, uno).await?;
        let mut house: HouseListingActiveModel = house_listing.into();
        house.reset(HouseListingColumn::Hdistrict);
        house.reset(HouseListingColumn::Haddr);
        house.reset(HouseListingColumn::Hlo);
        house.reset(HouseListingColumn::Hflr);
        house.reset(HouseListingColumn::Harea);
        house.reset(HouseListingColumn::Hprice);
        house.reset(HouseListingColumn::Hsuite);
        house.reset(HouseListingColumn::Hunlisted);
        println!("To Modify: {}, {:?}", uno, house);
        let house = HouseListingEntity::update(house).exec(db).await?;
        println!("Modify house by {}: {:?}", uno, house);
        Ok(house.hno)
    }
}
