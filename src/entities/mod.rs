use gamelibrary::traits::HasOwner;
use diff::Diff;
use physics_square::PhysicsSquare;
use serde::{Deserialize, Serialize};

use crate::traits::{IsClient, Tickable};

pub mod physics_square;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum Entity {
    PhysicsSquare(PhysicsSquare)
}

impl HasOwner for Entity {
    fn get_owner(&self) -> String {

        match self {
            Entity::PhysicsSquare(physics_square) => physics_square.get_owner()
        }
    }

    fn set_owner(&mut self, uuid: String) {
        match self {
            Entity::PhysicsSquare(physics_square) => physics_square.owner = uuid
        }
    }
}

impl Tickable for Entity {
    fn tick(&mut self, client: &mut dyn IsClient) {
        match self {
            Entity::PhysicsSquare(physics_square) => physics_square.tick(client)
            
        }
    }
}

impl From<PhysicsSquare> for Entity {
    fn from(value: PhysicsSquare) -> Self {
        Self::PhysicsSquare(value)
    }
}