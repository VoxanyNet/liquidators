use std::time::{SystemTime, UNIX_EPOCH};

pub mod game;
pub mod entities;
pub mod timeline;
pub mod game_state;
pub mod proxies;
pub mod time;
pub mod networking;
pub mod rigid_body;
pub mod collider;
pub mod space;

pub fn uuid() -> String {
    // AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
    macroquad::rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).expect("we went back in time!").as_nanos() as u64);
    macroquad::rand::rand().to_string()
}