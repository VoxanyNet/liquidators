use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::{level::Level};

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub level: Level
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            level: Level::empty()
        }
    }

}