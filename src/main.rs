use std::{collections::HashMap, time::{Duration, Instant}};

use game::Game;
use macroquad::{math::{Rect, Vec2}, miniquad::conf::Platform, window::{next_frame, Conf}};
use player::Player;
use crate::coin::Coin;

mod player;
mod game;
mod zombie;
mod bullet;
mod coin;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Kruz's Epic Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(0);
    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    //macroquad::window::set_fullscreen(true);
    
    let mut game = Game {
        textures: HashMap::new(),
        players: vec![
            Player {
                rect: Rect {x: 30.0, y: 30.0, w: 65.0, h: 100.0},
                scale: Vec2::new(5.0, 5.0),
                acceleration: 0.01,
                texture_path: "assets/player.png".to_string(),
                velocity: Vec2{x: 0.0, y: 0.0},
                health: 100,
                friction_coefficient: 0.02,
                up_bind: macroquad::input::KeyCode::W,
                down_bind: macroquad::input::KeyCode::S,
                left_bind: macroquad::input::KeyCode::A,
                right_bind: macroquad::input::KeyCode::D
            },

            Player {
                rect: Rect {x: 300.0, y: 300.0, w: 65.0, h: 100.0},
                scale: Vec2::new(5.0, 5.0),
                acceleration: 0.01,
                texture_path: "assets/player.png".to_string(),
                velocity: Vec2{x: 0.0, y: 0.0},
                health: 100,
                friction_coefficient: 0.02,
                up_bind: macroquad::input::KeyCode::Up,
                down_bind: macroquad::input::KeyCode::Down,
                left_bind: macroquad::input::KeyCode::Left,
                right_bind: macroquad::input::KeyCode::Right
            }
        ],
        zombies: vec![],
        coins: vec![Coin::new(500.0, 500.0)],
        dt: Duration::from_millis(1)
    };

    let mut last_tick = Instant::now();

    loop {

        
        macroquad::window::clear_background(macroquad::color::BLACK);
        

        game.tick();
        
        game.draw().await;

        next_frame().await;

        // cap framerate at 200fps (or 5 ms per frame)
        // TODO: this needs to take into account the time it took to draw the last frame
        std::thread::sleep(
            Duration::from_millis(5)
        );

        //println!("{}",  macroquad::time::get_fps());

        game.dt = last_tick.elapsed();

        last_tick = Instant::now();
    }
    
}