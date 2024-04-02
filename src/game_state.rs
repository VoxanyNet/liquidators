use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::entities::Entity;

#[derive(Serialize, Deserialize, Diff, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub entities: Vec<Entity>
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            entities: vec![]
        }
    }

}