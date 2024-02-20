use std::time::{Duration, Instant};

use game::Game;
use macroquad::{color::{RED, WHITE}, math::{Rect, Vec2}, miniquad::conf::Platform, window::{next_frame, Conf}};
use player::Player;

mod player;
mod game;
mod zombie;
mod bullet;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "App Template".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(-1);
    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    macroquad::window::set_fullscreen(true);

    let mut game = Game {
        players: vec![
            Player {
                rect: Rect {x: 30.0, y: 30.0, w: 30.0, h: 30.0},
                color: WHITE,
                velocity: Vec2{x: 0.0, y: 0.0},
                up_bind: macroquad::input::KeyCode::W,
                down_bind: macroquad::input::KeyCode::S,
                left_bind: macroquad::input::KeyCode::A,
                right_bind: macroquad::input::KeyCode::D
            },

            Player {
                rect: Rect {x: 90.0, y: 90.0, w: 30.0, h: 30.0},
                color: RED,
                velocity: Vec2{x: 0.0, y: 0.0},
                up_bind: macroquad::input::KeyCode::Up,
                down_bind: macroquad::input::KeyCode::Down,
                left_bind: macroquad::input::KeyCode::Left,
                right_bind: macroquad::input::KeyCode::Right
            }
        ],
        zombies: vec![],
        dt: Duration::from_millis(1)
    };

    let mut last_tick = Instant::now();

    loop {

        
        macroquad::window::clear_background(macroquad::color::BLACK);
        

        game.tick();
        
        game.draw();

        let start = Instant::now();

        next_frame().await;

        println!("{}",  macroquad::time::get_fps());

        game.dt = last_tick.elapsed();

        last_tick = Instant::now();
    }
    
}