#![feature(const_trait_impl)]
#![feature(effects)]
use core::panic;
use std::{collections::HashMap, os::linux::raw::stat, time::{Duration, Instant}};

use game::Game;
use game_state::GameState;
use macroquad::{miniquad::conf::Platform, window::{next_frame, Conf}};
use entities::{coin::Coin, player::Player, tree::Tree};
use crate::proxies::macroquad::math::rect::Rect;


mod game;
mod entities;
mod timeline;
mod game_state;
mod proxies;
mod time;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Kruz's Epic Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(-1); // disable vsync
    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    // macroquad::window::set_fullscreen(true);
    
    let mut game = Game::host();

    loop {
        
        //macroquad::window::clear_background(macroquad::color::BLACK);

        game.tick();
        
        game.draw().await;

        next_frame().await;

        if macroquad::input::is_key_down(macroquad::input::KeyCode::F4) {
            let state_string = serde_json::to_string(&game.game_state).unwrap();

            std::fs::write("state.json", state_string).expect("failed to write current state to state.json")
        }

        // cap framerate at 200fps (or 5 ms per frame)
        // TODO: this needs to take into account the time it took to draw the last frame
        std::thread::sleep(
            Duration::from_millis(5)
        );

        //println!("{}",  macroquad::time::get_fps());
    }
    
}