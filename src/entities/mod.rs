use gamelibrary::traits::HasOwner;
use diff::Diff;
use physics_square::PhysicsSquare;
use serde::{Deserialize, Serialize};

use crate::TickContext;

pub mod physics_square;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum Entity {
    PhysicsSquare(PhysicsSquare)
}

impl Entity {
    pub fn tick(&mut self, ctx: &mut TickContext) {
        match self {
            Entity::PhysicsSquare(physics_square) => physics_square.tick(ctx)
        }
    }
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

impl From<PhysicsSquare> for Entity {
    fn from(value: PhysicsSquare) -> Self {
        Self::PhysicsSquare(value)
    }
}