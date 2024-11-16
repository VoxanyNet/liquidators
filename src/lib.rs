use std::time::Instant;

use console::Console;
use diff::Diff;
use gamelibrary::texture_loader::TextureLoader;
use gilrs::{GamepadId, Gilrs};
use macroquad::math::{Rect, Vec2};
use serde::{Deserialize, Serialize};

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
pub mod portal;
pub mod portal_bullet;
pub mod portal_gun;
pub mod console;


pub struct TickContext<'a> {
    pub is_host: &'a mut bool,
    pub textures: &'a mut TextureLoader,
    pub last_tick: &'a web_time::Instant,
    pub uuid: &'a String,
    pub camera_rect: &'a Rect,
    pub camera_offset: &'a mut Vec2,
    pub active_gamepad: &'a Option<GamepadId>,
    pub console: &'a mut Console
}

#[derive(PartialEq, Serialize, Deserialize, Diff)]
pub struct SoundHandle {
    id: i32
}