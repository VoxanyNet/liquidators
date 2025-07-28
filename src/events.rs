use serde::{Deserialize, Serialize};

use crate::weapon::WeaponFireEvent;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Event {
    WeaponFire(WeaponFireEvent)
}