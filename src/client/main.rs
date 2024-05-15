use std::time::Duration;

use game::{collider::Collider, entities::player::Player, proxies::macroquad::math::vec2::Vec2, rigid_body::RigidBody, space::Space};
use macroquad::{miniquad::conf::Platform, window::Conf};
use client::Client;
use rapier2d::dynamics::RigidBodyBuilder;

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

    // let mut space = Space::new();

    // let mut collider = Collider { 
    //     hx: 10., 
    //     hy: 10., 
    //     restitution: 0., 
    //     mass: 10., 
    //     owner: "uranium fever".to_string()
    // };

    // let mut collider_two = Collider { 
    //     hx: 10., 
    //     hy: 10., 
    //     restitution: 0., 
    //     mass: 10., 
    //     owner: "uranium fever".to_string()
    // };

    // let mut rigid_body = RigidBody {
    //     position: Vec2::new(100., 0.),
    //     velocity: Vec2::new(0., 0.),
    //     body_type: game::rigid_body::RigidBodyType::Dynamic,
    //     owner: "uranium feverr".to_string(),
    //     collider: collider
    // };

    // let mut rigid_body_two = RigidBody {
    //     position: Vec2::new(0., 0.),
    //     velocity: Vec2::new(1000., 0.),
    //     body_type: game::rigid_body::RigidBodyType::Dynamic,
    //     owner: "uranium fever".to_string(),
    //     collider: collider_two
    // };

    // let rigid_body_handle = space.insert_rigid_body(rigid_body);
    // let rigid_body_handle_two = space.insert_rigid_body(rigid_body_two);

    // loop {
        
    //     std::thread::sleep(Duration::from_secs(1));

    //     space.step(&"uranium fever".to_string());
        
    //     {
    //         let rigid_body = space.get_rigid_body(&rigid_body_handle).expect("im so confused");
    //         println!("Rigid body one: ");
    //         println!("{}, {}", rigid_body.position.x, rigid_body.position.y);
    //     }

    //     {

    //         let rigid_body_two = space.get_rigid_body(&rigid_body_handle_two).expect("im never confused");
    //         println!("Rigid body two: ");
    //         println!("{}, {}", rigid_body_two.position.x, rigid_body_two.position.y);

    //     }
        

    // }

    let mut client = Client::connect("127.0.0.1:5556");

    client.game_state.entities.push(
        game::entities::Entity::Player(Player::new(client.uuid.clone()))
    );

    match serde_json::to_string_pretty(&client.game_state) {
        Ok(string) => println!("{}", string),
        Err(_) => panic!()
    }

    client.run().await;

}