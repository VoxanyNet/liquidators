
use game::{collider::Collider, entities::{physics_square::PhysicsSquare, player::Player}, proxies::macroquad::{color::colors::WHITE, math::vec2::Vec2}, rigid_body::RigidBody};
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
        game::entities::Entity::Player(Player::new(client.uuid.clone()))
    );

    let physics_square = PhysicsSquare::new(
        &mut client.game_state.space,
        Vec2::new(50., 500.),
        game::rigid_body::RigidBodyType::Fixed,
        20., 
        20., 
        &client.uuid,
        true
    );

    client.game_state.entities.push(game::entities::Entity::PhysicsSquare(physics_square));

    let physics_square = PhysicsSquare::new(
        &mut client.game_state.space,
        Vec2::new(50., 500.),
        game::rigid_body::RigidBodyType::Dynamic,
        20., 
        20., 
        &client.uuid,
        false
    );

    client.game_state.entities.push(game::entities::Entity::PhysicsSquare(physics_square));

    client.run().await;

}