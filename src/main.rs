use std::{collections::HashMap, time::{Duration, Instant}};

use game::Game;
use macroquad::{math, miniquad::conf::Platform, window::{next_frame, Conf}};
use entities::{coin::Coin, player::Player, tree::Tree};


mod game;
mod entities;
mod timeline;
mod game_state;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Kruz's Epic Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(0); // disable vsync
    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    // macroquad::window::set_fullscreen(true);

    // download assets
    
    
    let mut game = Game {
        textures: HashMap::new(),
        sounds: HashMap::new(),
        entities: vec![
            Player::new().into(),
            Tree::new(math::Rect::new(400., 400., 100., 200.)).into(),
            Coin::new(500., 500.).into()
        ],
        last_tick: Instant::now()
    };

    loop {
        
        //macroquad::window::clear_background(macroquad::color::BLACK);

        game.tick();
        
        game.draw().await;

        next_frame().await;

        // cap framerate at 200fps (or 5 ms per frame)
        // TODO: this needs to take into account the time it took to draw the last frame
        std::thread::sleep(
            Duration::from_millis(5)
        );

        //println!("{}",  macroquad::time::get_fps());
    }
    
}