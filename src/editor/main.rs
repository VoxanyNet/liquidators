use gamelibrary::{collider::Collider, proxies::macroquad::math::vec2::Vec2, rigid_body::RigidBody, space::Space};
use liquidators_lib::{level::Level, structure::Structure};
use macroquad::{color::WHITE, input::{self, is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, mouse_position}, miniquad::conf::Platform, window::{screen_height, Conf}};
use gamelibrary::traits::HasRigidBody;

pub mod menu;

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

    
    loop {

        // spawn square structure at mouse position
        if is_key_pressed(input::KeyCode::E) {

            let rigid_body_handle = level.space.insert_rigid_body(
                RigidBody { 
                    position: Vec2::new(mouse_position().0 - 20., (mouse_position().1 * -1. + screen_height()) - 20.), 
                    rotation: 0., 
                    angular_velocity: 0.,
                    velocity: Vec2::ZERO, 
                    body_type: gamelibrary::rigid_body::RigidBodyType::Dynamic, 
                    owner: "host".to_string(), 
                    collider: Collider { 
                        hx: 20., 
                        hy: 20., 
                        restitution: 0., 
                        mass: 10., 
                        owner: "host".to_string() 
                    }
                }
            );

            let new_structure = Structure { 
                rigid_body_handle: rigid_body_handle 
            };
            
            level.structures.push(new_structure);

        }

        for structure in &mut level.structures {
            structure.draw(&Vec2::new(0., 0.), &level.space).await
        }

        if is_key_down(input::KeyCode::F) {
            level.space.step(&"host".to_string());
        }
        

        macroquad::window::next_frame().await;
        
        
    }
    
    
}