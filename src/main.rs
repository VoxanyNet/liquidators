use std::time::{Duration, Instant};

use game::Game;
use macroquad::{color::WHITE, math::{Rect, Vec2}, window::next_frame};
use player::Player;

mod player;
mod game;
mod zombie;

#[macroquad::main("Test")]
async fn main() {
    let mut game = Game {
        players: vec![
            Player {
                rect: Rect {x: 30.0, y: 30.0, w: 30.0, h: 30.0},
                color: WHITE,
                velocity: Vec2{x: 0.0, y: 0.0}
            }
        ],
        zombies: vec![],
        dt: Duration::from_millis(1)
    };

    let mut last_tick = Instant::now();

    loop {

        
        macroquad::window::clear_background(macroquad::color::BLACK);

        next_frame().await;

        game.dt = last_tick.elapsed();

        last_tick = Instant::now();
    }
    
}