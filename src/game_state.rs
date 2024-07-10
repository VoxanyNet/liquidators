use gamelibrary::space::Space;
use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::physics_square::PhysicsSquare;

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub physics_squares: Vec<PhysicsSquare>,
    pub space: Space
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            physics_squares: vec![],
            space: Space::new()
        }
    }

}