use std::collections::HashMap;

use game_state::GameState;
use gamelibrary::time::Time;
use macroquad::{math::Vec2, texture::Texture2D};

pub mod game_state;
pub mod physics_square;
pub mod level;
pub mod structure;

pub struct TickContext<'a> {
    pub game_state: &'a mut GameState,
    pub is_host: &'a mut bool,
    pub textures: &'a mut HashMap<String, Texture2D>,
    pub sounds: &'a mut HashMap<String, macroquad::audio::Sound>,
    pub time: &'a Time,
    pub uuid: &'a String,
    pub camera_offset: &'a mut Vec2
}