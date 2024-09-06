use std::{collections::HashMap, fs, hash::RandomState, time::Instant};

use diff::Diff;
use gamelibrary::{space::Space, texture_loader::TextureLoader, time::Time};
use macroquad::{audio::Sound, input::{is_key_down, is_key_released, KeyCode}, math::Vec2};
use rapier2d::prelude::{collider, ColliderHandle, RigidBody, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{game_state::GameState, player::Player, radio::Radio, structure::Structure, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Level {
    pub structures: Vec<Structure>,
    pub players: Vec<Player>,
    pub space: Space,
    pub radios: Vec<Radio> 
}

impl Level {
    pub fn empty() -> Self {
        let mut level = Level { 
            structures: vec![],
            players: vec![],
            space: Space::new(),
            radios: vec![]
        };
    
        level.space.gravity.y = -980.;
        
        level
    }

    pub fn from_save(path: String) -> Self {
        
        let bytes = fs::read(path).unwrap();

        let level: Self = bitcode::deserialize(&bytes).unwrap();
        
        level
    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext,
    ) {
        
        let mut owned_bodies: Vec<RigidBodyHandle> = vec![];
        let mut owned_colliders: Vec<ColliderHandle> = vec![];

        for player_index in 0..self.players.len() {

            let mut player = self.players.remove(player_index);

            if player.owner == *ctx.uuid {
                player.tick(self, ctx);

                owned_bodies.push(player.rigid_body);
                owned_colliders.push(player.collider);
            }            

            self.players.insert(player_index, player);
        }

        for structure_index in 0..self.structures.len() {
            let structure = self.structures.remove(structure_index);

            match &structure.owner {
                Some(owner) => {
                    if owner == ctx.uuid {
                        owned_bodies.push(structure.rigid_body_handle);
                        owned_colliders.push(structure.collider_handle);
                    }
                },
                None => {},
            }

            self.structures.insert(structure_index, structure);
        }  

        self.space.step(&owned_bodies, &owned_colliders);
        
        


    }
}