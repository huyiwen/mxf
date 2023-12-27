use rocket::form::FromForm;
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serialize;
use sea_orm::{ColumnTrait, Condition};
use std::convert::From;

use crate::HouseListingColumn;

#[derive(FromForm, Default, Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct HouseFilter<'r> {
    #[field(name = "q", default = "")]
    _district: &'r str,

    #[field(name = "f", default = "")]
    _house_type: &'r str,

    #[field(name = "c", default = 0, validate = range(0..=6))]
    floor_enum: usize,

    #[field(name = "m", default = 0, validate = range(0..=6))]
    area_enum: usize,

    #[field(name = "bm")]
    _area_lower: Option<u32>,

    #[field(name = "em")]
    _area_upper: Option<u32>,

    #[field(name = "bp")]
    _price_lower: Option<u32>,

    #[field(name = "ep")]
    _price_upper: Option<u32>,

    #[field(name = "s", default = "")]
    _suite: &'r str,

    #[field(default = 1, validate = range(1..))]
    pub page: u64,
}

impl HouseFilter<'_> {
    const AREAS: [u32; 6] = [0, 50, 70, 90, 110, 150];
    const FLOORS: [u32; 6] = [0, 1, 10, 20, 30, 31];
    const CHECKED: &'static str = "checked";

    pub fn district(&self) -> Option<&str> {
        if self._district.len() > 0 {
            Some(self._district)
        } else {
            None
        }
    }

    pub fn house_type(&self) -> Option<&str> {
        if self._house_type.len() > 0 {
            Some(self._house_type)
        } else {
            None
        }
    }

    pub fn suite(&self) -> Option<&str> {
        if self._suite.len() > 0 {
            Some(self._suite)
        } else {
            None
        }
    }

    pub fn area_lower(&self) -> Option<u32> {
        if let Some(area_lower) = self._area_lower {
            Some(area_lower)
        } else if 0 < self.area_enum && self.area_enum < Self::AREAS.len() {
            Some(Self::AREAS[self.area_enum - 1])
        } else {
            None
        }
    }

    pub fn area_upper(&self) -> Option<u32> {
        if let Some(area_upper) = self._area_upper {
            Some(area_upper)
        } else if 0 < self.area_enum && self.area_enum < Self::AREAS.len() {
            Some(Self::AREAS[self.area_enum])
        } else {
            None
        }
    }

    pub fn floor_lower(&self) -> Option<u32> {
        if 0 < self.floor_enum && self.floor_enum < Self::FLOORS.len() {
            Some(Self::FLOORS[self.floor_enum - 1])
        } else {
            None
        }
    }

    pub fn floor_upper(&self) -> Option<u32> {
        if 0 < self.floor_enum && self.floor_enum < Self::FLOORS.len() - 1 {
            Some(Self::FLOORS[self.floor_enum])
        } else {
            None
        }
    }

    pub fn price_lower(&self) -> Option<u32> {
        self._price_lower
    }

    pub fn price_upper(&self) -> Option<u32> {
        self._price_upper
    }

    fn floor_checked(&self) -> Vec<&str> {
        let mut fc = vec![""; Self::FLOORS.len()];
        fc[self.floor_enum] = Self::CHECKED;
        fc
    }

    fn area_checked(&self) -> Vec<&str> {
        let mut fc = vec![""; Self::AREAS.len()];
        if self._area_lower.is_none() && self._area_upper.is_none() {
            fc[self.area_enum] = Self::CHECKED;
        }
        fc
    }
}

impl From<HouseFilter<'_>> for Condition {
    fn from(value: HouseFilter) -> Self {
        Condition::all()
            .add_option(
                value
                    .floor_lower()
                    .map(|fl| HouseListingColumn::Hflr.gte(fl)),
            )
            .add_option(
                value
                    .floor_upper()
                    .map(|fu| HouseListingColumn::Hflr.lt(fu)),
            )
            .add_option(
                value
                    .area_lower()
                    .map(|al| HouseListingColumn::Harea.gte(al)),
            )
            .add_option(
                value
                    .area_upper()
                    .map(|au| HouseListingColumn::Harea.lt(au)),
            )
            .add_option(
                value
                    .price_lower()
                    .map(|pl| HouseListingColumn::Hprice.gte(pl)),
            )
            .add_option(
                value
                    .price_upper()
                    .map(|pu| HouseListingColumn::Hprice.lt(pu)),
            )
            .add_option(
                value
                    .district()
                    .map(|d| HouseListingColumn::Hdistrict.contains(d)),
            )
            .add_option(
                value
                    .house_type()
                    .map(|ht| HouseListingColumn::Hlo.contains(ht)),
            )
            .add_option(
                value
                    .suite()
                    .map(|s| HouseListingColumn::Hsuite.contains(s)),
            )
    }
}

impl Serialize for HouseFilter<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("HouseFilter", 10)?;
        s.serialize_field("q", &self.district())?;
        s.serialize_field("f", &self.house_type())?;
        s.serialize_field("c", &self.floor_checked())?;
        s.serialize_field("m", &self.area_checked())?;
        s.serialize_field("bm", &self._area_lower)?;
        s.serialize_field("em", &self._area_upper)?;
        s.serialize_field("bp", &self._price_lower)?;
        s.serialize_field("ep", &self._price_upper)?;
        s.serialize_field("s", &self.suite())?;
        s.serialize_field("page", &self.page)?;
        s.end()
    }
}

impl std::fmt::Display for HouseFilter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = Vec::<String>::new();
        if let Some(district) = self.district() {
            repr.push(format!("district: {}", district));
        }
        if let Some(house_type) = self.house_type() {
            repr.push(format!("house_type: {}", house_type));
        }
        if let Some(area_lower) = self.area_lower() {
            repr.push(format!("area_lower: {}", area_lower));
        }
        if let Some(area_upper) = self.area_upper() {
            repr.push(format!("area_upper: {}", area_upper));
        }
        if let Some(floor_lower) = self.floor_lower() {
            repr.push(format!("floor_lower: {}", floor_lower));
        }
        if let Some(floor_upper) = self.floor_upper() {
            repr.push(format!("floor_upper: {}", floor_upper));
        }
        if let Some(suite) = self.suite() {
            repr.push(format!("suite: {}", suite));
        }
        // repr.push(format!("page: {}", self.page));
        write!(f, "HouseFilter({})", repr.join(", "))?;
        Ok(())
    }
}
