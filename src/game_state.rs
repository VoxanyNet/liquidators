use diff::Diff;
use serde::{Deserialize, Serialize};
use crate::entities::coin::Coin;
use crate::entities::player::Player;

use crate::entities::tree::Tree;
use crate::entities::Entity;
use crate::proxies::macroquad::math::rect::Rect;
use crate::proxies::uuid::lib::Uuid;

#[derive(Serialize, Deserialize, Diff, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub entities: Vec<Entity>
}

impl GameState {

    pub fn host(owner_uuid: Uuid) -> Self {
        Self {
            entities: vec![
                Player::new(owner_uuid).into(),
                Tree::new(Rect::new(400., 400., 100., 200.), owner_uuid).into(),
                Coin::new(500., 500., owner_uuid).into()
            ]
        }
    }

}