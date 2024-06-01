use std::collections::HashMap;

use game_state::GameState;
use gamelibrary::{proxies::macroquad::math::vec2::Vec2, time::Time};
use macroquad::{texture::Texture2D, window::screen_height};

pub mod game_state;
pub mod physics_square;
pub mod level;
pub mod structure;

pub fn translate_coordinates(macroquad_coords: &Vec2) -> Vec2 {
    // translate macroquad coords to rapier coords
    Vec2 { 
        x: macroquad_coords.x, 
        y: (macroquad_coords.y * -1.) + screen_height()
    }
}

pub struct TickContext<'a> {
    pub game_state: &'a mut GameState,
    pub is_host: &'a mut bool,
    pub textures: &'a mut HashMap<String, Texture2D>,
    pub sounds: &'a mut HashMap<String, macroquad::audio::Sound>,
    pub time: &'a Time,
    pub uuid: &'a String,
    pub camera_offset: &'a mut Vec2
}