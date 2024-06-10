use gamelibrary::{collider::Collider, proxies::macroquad::{color::colors::{DARKGRAY, RED}, math::vec2::Vec2}, rigid_body::RigidBody};
use liquidators_lib::{level::Level, physics_square::PhysicsSquare, structure::Structure};
use macroquad::{input::{self, is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, mouse_position}, window::screen_height};
use gamelibrary::traits::HasRigidBody;

pub struct Editor {
    pub level: Level
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 


impl Editor {

    pub fn spawn_structure(&mut self) {
        if is_key_pressed(input::KeyCode::E) {

            let rigid_body_handle = self.level.space.insert_rigid_body(
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
                rigid_body_handle: rigid_body_handle,
                color: RED
            };
            
            self.level.structures.push(new_structure);

        }
    }

    pub fn step_space(&mut self) {
        if is_key_down(input::KeyCode::F) {
            self.level.space.step(&"host".to_string());
        }
    }

    pub fn tick(&mut self) {

        // spawn square structure at mouse position
        self.spawn_structure();
        
        self.step_space();
        
    }

    pub async fn draw(&mut self) {

        for structure in &mut self.level.structures {
            structure.draw(&Vec2::new(0., 0.), &self.level.space).await
        }

    }

    pub async fn run(&mut self) {

        loop {
            self.tick();

            self.draw().await;

            macroquad::window::next_frame().await;
        }

    }
}