use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HnoData {
    pub hno: u32,
}
