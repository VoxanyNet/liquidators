use std::{collections::HashMap, hash::RandomState};

use diff::Diff;
use gamelibrary::{space::Space, texture_loader::TextureLoader, time::Time};
use macroquad::{audio::Sound, math::Vec2};
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

    pub fn tick(
        &mut self,
        ctx: &mut TickContext,
        ) {
        for player_index in 0..self.players.len() {

            let mut player = self.players.remove(player_index);

            player.tick(self, ctx);

            self.players.insert(player_index, player);
        }
    }
}