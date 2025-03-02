
use std::{fs, panic, time::{Duration, Instant}};

use gamelibrary::space::Space;
use liquidators_lib::game_state::GameState;
use macroquad::{input::{is_key_released, prevent_quit, show_mouse}, miniquad::conf::Platform, window::Conf};
use client::Client;
use nalgebra::vector;
use rapier2d::prelude::{rigid_body, ColliderBuilder, RigidBody, RigidBodyBuilder};

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
    conf.platform.swap_interval = Some(-1); // disable vsync

    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut client = Client::connect("ws://127.0.0.1:5556").await;

    client.run().await;

}