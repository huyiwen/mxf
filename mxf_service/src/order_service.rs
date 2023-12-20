use chrono::{Duration, Utc};
use sea_orm::sea_query::Query;
use sea_orm::*;

use mxf_entity::{MXFError, OrderActiveModel, OrderColumn, OrderEntity, OrderModel, OrderType};

pub struct OrderService {
    date_format: &'static str,
}

impl OrderService {
    pub fn init() -> Self {
        Self {
            date_format: "%Y-%m-%d",
        }
    }

    pub async fn get_orders_by_hno(
        &self,
        db: &DbConn,
        hno: u32,
        confirmed_only: bool,
    ) -> Result<Vec<OrderModel>, MXFError> {
        if confirmed_only {
            OrderEntity::find()
                .filter(OrderColumn::Hno.eq(hno))
                .filter(
                    Condition::any()
                        .add(OrderColumn::Otype.eq(OrderType::LeaseConfirm))
                        .add(OrderColumn::Otype.eq(OrderType::CancelConfirm)),
                )
                .all(db)
                .await
                .map_err(|e| e.into())
        } else {
            OrderEntity::find()
                .filter(OrderColumn::Hno.eq(hno))
                .all(db)
                .await
                .map_err(|e| e.into())
        }
    }

    pub async fn get_open_orders_by_uno(
        &self,
        db: &DbConn,
        uno: u32,
    ) -> Result<Vec<OrderModel>, MXFError> {
        OrderEntity::find()
            .filter(
                Condition::any()
                    .add(OrderColumn::Hlandlore.eq(uno))
                    .add(OrderColumn::Htenant.eq(uno)),
            )
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_orders(&self, db: &DbConn) -> Result<Vec<OrderModel>, MXFError> {
        OrderEntity::find().all(db).await.map_err(|e| e.into())
    }

    pub async fn check_available(&self, db: &DbConn, hno: u32) -> Result<u32, MXFError> {
        let orders = self.get_orders_by_hno(db, hno, true).await?;

        let mut max_ono = 0;
        let mut max_otype = OrderType::LeaseRequest;
        for order in orders {
            if order.ono > max_ono {
                max_ono = order.ono;
                max_otype = order.otype;
            } else if order.ono == max_ono {
                if order.otype > max_otype {
                    max_otype = order.otype;
                }
            }
        }

        if max_otype == OrderType::LeaseConfirm {
            Err(MXFError::HouseUnavailable(hno))
        } else {
            self.get_next_ono(db).await
        }
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
        ono: u32,
        hno: u32,
        hlandlore: u32,
        htenant: u32,
    ) -> Result<(), MXFError> {
        let now = Utc::now();
        let oend = now
            .checked_add_signed(Duration::weeks(4))
            .ok_or(MXFError::UnknownError("end of the world error".into()))?;
        let order = OrderActiveModel {
            ono: Set(ono),
            hno: Set(hno),
            hlandlore: Set(hlandlore),
            htenant: Set(htenant),
            odate: Set(now.to_rfc3339()),
            otype: Set(OrderType::LeaseRequest),
            ostart: Set(now.format(self.date_format).to_string()),
            oend: Set(oend.format(self.date_format).to_string()),
            ostatus: Set(ono),
        };
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
            return Err(MXFError::NonOwnerCannotConfirm);
        }

        let next_ono = self.get_next_ono(db).await?;
        let now = Utc::now();
        let order = OrderActiveModel {
            ono: Set(next_ono),
            hno: Set(order.hno),
            hlandlore: Set(order.hlandlore),
            htenant: Set(order.htenant),
            odate: Set(now.to_rfc3339()),
            otype: Set(OrderType::LeaseConfirm),
            ostart: Set(order.ostart),
            oend: Set(order.oend),
            ostatus: Set(ono),
        };
        order.insert(db).await?;
        Ok(())
    }
}
