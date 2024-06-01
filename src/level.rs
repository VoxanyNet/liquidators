use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::{physics_square::PhysicsSquare, structure::Structure};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Level {
    pub physics_squares: Vec<PhysicsSquare>,
    pub structures: Vec<Structure>
}