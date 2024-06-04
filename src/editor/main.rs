use editor::Editor;
use gamelibrary::{collider::Collider, proxies::macroquad::{color::colors::RED, math::vec2::Vec2}, rigid_body::RigidBody, space::Space};
use liquidators_lib::{level::Level, structure::Structure};
use macroquad::{color::WHITE, input::{self, is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, mouse_position}, miniquad::conf::Platform, window::{screen_height, Conf}};
use gamelibrary::traits::HasRigidBody;

pub mod menu;
pub mod editor;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Liquidators Level Editor".to_owned(),
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

    let mut level = Level { 
        physics_squares: vec![], 
        structures: vec![],
        space: Space::new(-980.)
    };

    
    let mut editor = Editor { 
        level 
    };

    editor.run().await;
    
    
}