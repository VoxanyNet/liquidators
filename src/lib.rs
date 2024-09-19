use std::{collections::HashMap, path::Iter, time::Instant};

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
pub mod chat;
pub mod vec_remove_iter;
pub mod brick;



pub struct TickContext<'a> {
    pub is_host: &'a mut bool,
    pub textures: &'a mut TextureLoader,
    pub last_tick: &'a Instant,
    pub sounds: &'a mut HashMap<String, macroquad::audio::Sound>,
    pub uuid: &'a String,
    pub camera_offset: &'a mut Vec2
}