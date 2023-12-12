use rocket::form::FromForm;
use rocket::serde::Serialize;
use serde::ser::SerializeStruct;

#[derive(Serialize)]
pub(super) struct IndexContext<'a> {
    q: &'a str,
    f: &'a str,
    c: Vec<&'a str>,
    m: Vec<&'a str>,
    bm: &'a str,
    em: &'a str,
    bp: &'a str,
    ep: &'a str,
    s: &'a str,
}

#[derive(FromForm)]
pub(super) struct HouseFilter<'r> {
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
}

impl HouseFilter<'_> {

    const AREAS: [u32; 6] = [0, 50, 70, 90, 110, 150];
    const FLOORS: [u32; 6] = [0, 1, 10, 20, 30, 31];
    const CHECKED: &'static str = "checked";

    pub(super) fn district(&self) -> Option<&str> {
        if self._district.len() > 0 {
            Some(self._district)
        } else {
            None
        }
    }

    pub(super) fn house_type(&self) -> Option<&str> {
        if self._house_type.len() > 0 {
            Some(self._house_type)
        } else {
            None
        }
    }

    pub(super) fn suite(&self) -> Option<&str> {
        if self._suite.len() > 0 {
            Some(self._suite)
        } else {
            None
        }
    }

    pub(super) fn area_lower(&self) -> Option<u32> {
        if let Some(area_lower) = self._area_lower {
            Some(area_lower)
        } else if 0 < self.area_enum && self.area_enum < Self::AREAS.len() {
            Some(Self::AREAS[self.area_enum - 1])
        } else {
            None
        }
    }

    pub(super) fn area_upper(&self) -> Option<u32> {
        if let Some(area_upper) = self._area_upper {
            Some(area_upper)
        } else if 0 < self.area_enum && self.area_enum < Self::AREAS.len() {
            Some(Self::AREAS[self.area_enum])
        } else {
            None
        }
    }

    pub(super) fn floor_lower(&self) -> Option<u32> {
        if 0 < self.floor_enum && self.floor_enum < Self::FLOORS.len() {
            Some(Self::FLOORS[self.floor_enum - 1])
        } else {
            None
        }
    }

    pub(super) fn floor_upper(&self) -> Option<u32> {
        if 0 < self.floor_enum && self.floor_enum < Self::FLOORS.len() - 1 {
            Some(Self::FLOORS[self.floor_enum])
        } else {
            None
        }
    }

    pub(super) fn price_lower(&self) -> Option<u32> {
        self._price_lower
    }

    pub(super) fn price_upper(&self) -> Option<u32> {
        self._price_upper
    }

    fn floor_checked(&self) -> Vec<&str> {
        let mut fc = vec![""; Self::FLOORS.len()];
        if self.floor_lower().is_none() && self.floor_upper().is_none() {
            fc[self.floor_enum] = Self::CHECKED;
        }
        fc
    }

    fn area_checked(&self) -> Vec<&str> {
        let mut fc = vec![""; Self::AREAS.len()];
        if self.area_lower().is_none() && self.area_upper().is_none() {
            fc[self.area_enum] = Self::CHECKED;
        }
        fc
    }
}

impl Serialize for HouseFilter<'_> {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("HouseFilter", 9)?;
        s.serialize_field("q", &self.district())?;
        s.serialize_field("f", &self.house_type())?;
        s.serialize_field("c", &self.floor_checked())?;
        s.serialize_field("m", &self.area_checked())?;
        s.serialize_field("bm", &self._area_lower)?;
        s.serialize_field("em", &self._area_upper)?;
        s.serialize_field("bp", &self._price_lower)?;
        s.serialize_field("ep", &self._price_upper)?;
        s.serialize_field("s", &self.suite())?;
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
        write!(f, "HouseFilter({})", repr.join(", "))?;
        Ok(())
    }
}