use chrono::{Duration, Utc};
use sea_orm::*;

use mxf_entity::{MXFError, OrderActiveModel, OrderColumn, OrderEntity, OrderType};

pub struct OrderService;

impl OrderService {
    pub fn init() -> Self {
        Self
    }

    pub async fn check_available(&self, db: &DbConn, hno: u32) -> Result<u32, MXFError> {
        let all_orders = OrderEntity::find()
            .column(OrderColumn::Ono)
            .filter(OrderColumn::Hno.eq(hno))
            .all(db)
            .await?;
        println!("all: {:?}", all_orders);

        let orders = OrderEntity::find()
            .filter(OrderColumn::Hno.eq(hno))
            .filter(
                Condition::any()
                    .add(OrderColumn::Otype.eq(OrderType::LeaseConfirm))
                    .add(OrderColumn::Otype.eq(OrderType::CancelConfirm)),
            )
            .all(db)
            .await?;

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

        // TODO: cache the next ono

        let next_ono = OrderEntity::find()
            .order_by_desc(OrderColumn::Ono)
            .one(db)
            .await?
            .unwrap()
            .ono
            + 1;

        if max_otype == OrderType::LeaseConfirm {
            Err(MXFError::HouseUnavailable(hno))
        } else {
            Ok(next_ono)
        }
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
            ostart: Set(now.to_rfc3339()),
            oend: Set(oend.to_rfc3339()),
            ostatus: Set(ono),
        };
        order.insert(db).await?;
        Ok(())
    }
}
