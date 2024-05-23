use std::{net::SocketAddr, str::FromStr};

use macroquad::{miniquad::conf::Platform, window::Conf};


pub mod server;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Kruz's Epic Game Server".to_owned(),
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
async fn main () {

    let mut server = server::Server::new(
        SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5556)
    );
    
    server.run().await;
}

