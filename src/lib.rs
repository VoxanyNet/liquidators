use std::time::Instant;

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


pub struct TickContext<'a> {
    pub is_host: &'a mut bool,
    pub textures: &'a mut TextureLoader,
    pub last_tick: &'a Instant,
    pub uuid: &'a String,
    pub camera_rect: &'a Rect,
    pub camera_offset: &'a mut Vec2,
    pub gilrs: &'a mut Gilrs,
    pub active_gamepad: &'a Option<GamepadId>
}

#[derive(PartialEq, Serialize, Deserialize, Diff)]
pub struct SoundHandle {
    id: i32
}