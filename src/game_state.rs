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

    pub fn tick(
        &mut self,
        ctx: &mut TickContext
    ) { 

        self.level.tick(ctx)

    }

}