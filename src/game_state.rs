use gamelibrary::space::Space;
use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::entities::Entity;

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub space: Space
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            entities: vec![],
            space: Space::new(0.)
        }
    }

}