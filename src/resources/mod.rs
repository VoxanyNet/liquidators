use self::{coin::Coin, wood::Wood};

pub mod coin;
pub mod wood;

pub enum Resource {
    Coin(Coin),
    Wood(Wood)
}