use std::{collections::HashMap, fs};

use diff::Diff;
use gamelibrary::{texture_loader::TextureLoader, time::Time};
use macroquad::{audio::Sound, math::{Rect, Vec2}};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{level::Level, TickContext};

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub level: Level,
    pub game_started: bool
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            level: Level::empty(),
            game_started: false
        }
    }

    pub fn from_save(path: String) -> Self {
        
        let bytes = fs::read(path).unwrap();

        let game_state: GameState = bitcode::deserialize(&bytes).unwrap();
        
        game_state
    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext
    ) { 
   
        self.step_space();

        self.level.tick(ctx)

    }

    pub fn step_space(&mut self) {

        let mut owned_colliders: Vec<ColliderHandle> = vec![];
        let mut owned_bodies: Vec<RigidBodyHandle> = vec![];

        for structure in &self.level.structures {
            owned_bodies.push(structure.rigid_body_handle);
            owned_colliders.push(structure.collider_handle);
        }

        self.level.space.step(&owned_bodies, &owned_colliders);
    }

}