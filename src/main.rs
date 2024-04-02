#![feature(const_trait_impl)]
#![feature(effects)]

use std::{net::{Ipv4Addr, SocketAddr}, str::FromStr};

use game::Game;
use macroquad::{color::WHITE, miniquad::conf::Platform, window::Conf};


mod game;
mod entities;
mod timeline;
mod game_state;
mod proxies;
mod time;
mod server;
mod networking;

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

    let arguments: Vec<String> = std::env::args().collect();

    if arguments[1] == "client" {

        // macroquad::window::set_fullscreen(true);
        
        let mut game = Game::connect("127.0.0.1:5556");

        game.run().await;
    }

    if arguments[1] == "server" {

        let mut server = server::Server::new(
            SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5556)
        );

        server.run();
    }
    
}