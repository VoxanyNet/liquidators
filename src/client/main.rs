use macroquad::{miniquad::conf::Platform, window::Conf};
use client::Client;

pub mod client;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        platform: Platform::default(),
        fullscreen: false,
        ..Default::default()
    };
    conf.platform.swap_interval = Some(0); // disable vsync

    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut client = Client::connect("ws://127.0.0.1:5556").await;

    client.run().await;

}