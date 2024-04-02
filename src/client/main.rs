use game::entities::player::Player;
use macroquad::{miniquad::conf::Platform, window::Conf};
use client::Client;

pub mod client;

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

    let mut client = Client::connect("127.0.0.1:5556");

    client.game_state.entities.push(
        game::entities::Entity::Player(Player::new(client.uuid))
    );

    match serde_json::to_string_pretty(&client.game_state) {
        Ok(string) => println!("{}", string),
        Err(_) => panic!()
    }

    client.run().await;

}