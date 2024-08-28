use std::collections::HashMap;

use game_state::GameState;
use gamelibrary::{texture_loader::TextureLoader, time::Time};
use macroquad::math::Vec2;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

pub mod game_state;
pub mod physics_square;
pub mod level;
pub mod structure;
pub mod shotgun;
pub mod player;
pub mod radio;

pub struct TickContext<'a> {
    pub is_host: &'a mut bool,
    pub textures: &'a mut TextureLoader,
    pub sounds: &'a mut HashMap<String, macroquad::audio::Sound>,
    pub time: &'a Time,
    pub uuid: &'a String,
    pub camera_offset: &'a mut Vec2
}