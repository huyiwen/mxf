use chrono::{Local, Months};
use sea_orm::*;
use std::collections::HashMap;

use mxf_entity::{MXFError, OrderActiveModel, OrderColumn, OrderEntity, OrderModel, OrderType};

pub struct OrderService;

impl OrderService {
    pub fn init() -> Self {
        Self {}
    }

    pub async fn get_orders_by_hno(
        &self,
        db: &DbConn,
        hno: u32,
    ) -> Result<Vec<OrderModel>, MXFError> {
        OrderEntity::find()
            .filter(OrderColumn::Hno.eq(hno))
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub fn filter_latest(orders: &Vec<OrderModel>) -> Vec<OrderModel> {
        let mut orders_type: HashMap<u32, OrderType> = HashMap::new();
        for order in orders {
            if orders_type
                .entry(order.ostatus)
                .or_insert(order.otype)
                .clone()
                < order.otype
            {
                orders_type.insert(order.ostatus, order.otype);
            }
        }
        orders
            .iter()
            .filter(|o| orders_type[&o.ostatus] == o.otype)
            .cloned()
            .collect()
    }

    pub async fn get_orders_by_htenant(
        &self,
        db: &DbConn,
        htenant: u32,
    ) -> Result<Vec<OrderModel>, MXFError> {
        OrderEntity::find()
            .filter(OrderColumn::Htenant.eq(htenant))
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_orders_by_hlandlore(
        &self,
        db: &DbConn,
        hlandlore: u32,
    ) -> Result<Vec<OrderModel>, MXFError> {
        OrderEntity::find()
            .filter(OrderColumn::Hlandlore.eq(hlandlore))
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_orders(&self, db: &DbConn) -> Result<Vec<OrderModel>, MXFError> {
        OrderEntity::find().all(db).await.map_err(|e| e.into())
    }

    pub async fn check_available(&self, db: &DbConn, hno: u32) -> Result<u32, MXFError> {
        let orders: Vec<(u32, u32)> = OrderEntity::find()
            .select_only()
            .column(OrderColumn::Ostatus)
            .column_as(OrderColumn::Otype.max(), "mtype")
            .filter(OrderColumn::Hno.eq(hno))
            .group_by(OrderColumn::Ostatus)
            .into_tuple()
            .all(db)
            .await
            .map_err(|e| MXFError::DatabaseError(e))?;

        println!("{:?}", orders);
        for order in orders {
            if order.1 == (OrderType::LeaseRequest as u32) {
                return Err(MXFError::HouseUnavailable(hno));
            }
        }

        self.get_next_ono(db).await
    }

    async fn get_next_ono(&self, db: &DbConn) -> Result<u32, MXFError> {
        // TODO: cache the next ono
        let next_ono = OrderEntity::find()
            .order_by_desc(OrderColumn::Ono)
            .one(db)
            .await?
            .unwrap()
            .ono
            + 1;
        Ok(next_ono)
    }

    pub async fn place_order_by_ono(
        &self,
        db: &DbConn,
        next_ono: u32,
        hno: u32,
        hlandlore: u32,
        htenant: u32,
    ) -> Result<(), MXFError> {
        let now = Local::now();
        let oend = now
            .checked_add_months(Months::new(1))
            .ok_or(MXFError::UnknownError("end of the world error".into()))?;
        let order = OrderActiveModel {
            ono: Set(next_ono),
            hno: Set(hno),
            hlandlore: Set(hlandlore),
            htenant: Set(htenant),
            odate: Set(now.naive_local()),
            otype: Set(OrderType::LeaseRequest),
            ostart: Set(now.naive_local()),
            oend: Set(oend.naive_local()),
            ostatus: Set(next_ono),
        };
        println!("Place order: {:?}", order);
        order.insert(db).await?;
        Ok(())
    }

    pub async fn confirm_order(
        &self,
        db: &DbConn,
        ono: u32,
        user_uno: u32,
    ) -> Result<(), MXFError> {
        let order = OrderEntity::find_by_id(ono).one(db).await?.unwrap();
        if order.hlandlore != user_uno {
            return Err(MXFError::NotLandlore(user_uno));
        }

        let now = Local::now();
        let order = OrderActiveModel {
            ono: NotSet,
            hno: Set(order.hno),
            hlandlore: Set(order.hlandlore),
            htenant: Set(order.htenant),
            odate: Set(now.naive_local()),
            otype: Set(OrderType::LeaseConfirm),
            ostart: Set(order.ostart),
            oend: Set(order.oend),
            ostatus: Set(ono),
        };
        order.insert(db).await?;
        Ok(())
    }
}
