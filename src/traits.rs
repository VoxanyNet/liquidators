use std::collections::HashMap;

use gamelibrary::{proxies::macroquad::math::vec2::Vec2, time::Time};
use macroquad::texture::Texture2D;

use crate::game_state::GameState;

pub trait IsClient {
    fn get_game_state(&mut self) -> &mut GameState;

    fn get_is_host(&mut self) -> &mut bool;

    fn get_textures(&mut self) -> &mut HashMap<String, Texture2D>;

    fn get_sounds(&mut self) -> &mut HashMap<String, macroquad::audio::Sound>;

    fn get_last_tick(&self) -> &Time;

    fn get_uuid(&mut self) -> &String;

    fn get_camera_offset(&mut self) -> &mut Vec2;
}

pub trait Tickable {
    fn tick(&mut self, client: &mut dyn IsClient);
}