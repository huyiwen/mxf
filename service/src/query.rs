use super::house_listing;
use super::house_listing::Entity as HouseListing;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_house_by_id(
        db: &DbConn,
        hno: u8,
    ) -> Result<Option<house_listing::Model>, DbErr> {
        HouseListing::find_by_id(hno).one(db).await
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_houses_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u8,
    ) -> Result<(Vec<house_listing::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = HouseListing::find()
            .order_by_asc(house_listing::Column::Hno)
            .paginate(db, posts_per_page as u64);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
