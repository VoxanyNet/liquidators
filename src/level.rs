use diff::Diff;
use gamelibrary::space::Space;
use serde::{Deserialize, Serialize};

use crate::{player::Player, structure::Structure};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Level {
    pub structures: Vec<Structure>,
    pub players: Vec<Player>,
    pub space: Space 
}

impl Level {
    pub fn empty() -> Self {
        let mut level = Level { 
            structures: vec![],
            players: vec![],
            space: Space::new()
        };
    
        level.space.gravity.y = -980.;
        
        level
    }
}