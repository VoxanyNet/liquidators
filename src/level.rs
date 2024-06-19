use diff::Diff;
use gamelibrary::space::Space;
use serde::{Deserialize, Serialize};

use crate::structure::Structure;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Level {
    pub structures: Vec<Structure>,
    pub space: Space
}