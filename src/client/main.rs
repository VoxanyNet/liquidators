use macroquad::{miniquad::conf::Platform, window::Conf};
use client::Client;

pub mod client;

#[cfg(feature = "3d-audio")]
use gamelibrary::sound::backends::ears::EarsSoundManager as SelectedSoundManager; // this alias needs a better name

#[cfg(not(feature = "3d-audio"))]
use gamelibrary::sound::backends::macroquad::MacroquadSoundManager as SelectedSoundManager;


fn window_conf() -> Conf {

    let mut platform = Platform::default();

    let mut conf = Conf {
        window_title: "Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        platform: Platform::default(),
        fullscreen: false,
        ..Default::default()
    };
    conf.platform.swap_interval = Some(0); // disable vsync

    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut client: Client<SelectedSoundManager> = Client::connect("ws://voxany.net:5556").await;

    client.run().await;

}