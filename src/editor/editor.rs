use std::time::Instant;

use gamelibrary::{collider::Collider, proxies::macroquad::{color::colors::{DARKGRAY, RED}, math::vec2::Vec2}, rigid_body::RigidBody};
use liquidators_lib::{level::Level, physics_square::{self, PhysicsSquare}, structure::Structure, translate_coordinates};
use macroquad::{input::{self, is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position}, window::screen_height};
use gamelibrary::traits::HasRigidBody;
use nalgebra::point;
use rapier2d::pipeline::{QueryFilter, QueryPipeline};

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
                color: RED,
                menu: None
            };
            
            self.level.structures.push(new_structure);

        }

        if is_key_pressed(input::KeyCode::R) {

            let rigid_body_handle = self.level.space.insert_rigid_body(
                RigidBody { 
                    position: Vec2::new(mouse_position().0 - 20., (mouse_position().1 * -1. + screen_height()) - 20.), 
                    rotation: 0., 
                    angular_velocity: 0.,
                    velocity: Vec2::ZERO, 
                    body_type: gamelibrary::rigid_body::RigidBodyType::Fixed, 
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
                color: RED,
                menu: None
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
        
        let now = Instant::now();
        self.step_space();
        println!("{:?}", now.elapsed());

        self.update_menus();

        self.handle_menus();

        self.spawn_menus();
        
    }

    pub fn handle_menus(&mut self) {

        let mut structure_index = 0;
        let mut structures_length = self.level.structures.len();

        loop {

            if structure_index >= structures_length {
                break;
            }

            let structure = self.level.structures.remove(structure_index);

            let result = structure.handle_menu();

            match result {
                Some(structure) => {
                    self.level.structures.insert(structure_index, structure);

                    structure_index += 1;
                },
                None => {
                    structures_length -= 1;

                    // we dont increment the index
                },
            };
            
        }
    
            
    }

    pub fn update_menus(&mut self) {
        for structure in &mut self.level.structures {
            match &mut structure.menu {
                Some(menu) => menu.update(),
                None => {},
            }
        }
    }
    pub fn spawn_menus(&mut self) {

        if !is_mouse_button_released(input::MouseButton::Right) {
            return;
        }

        println!("mouse released");

        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        let intersections = self.level.space.query_point(translate_coordinates(&mouse_pos));
        
        println!("{:?}", intersections);

        for structure in &mut self.level.structures {

            if intersections.contains(structure.get_rigid_body_handle()) {
                println!("spawning");
                structure.spawn_menu(mouse_pos);
            }

        }
    }

    pub async fn draw(&mut self) {

        for structure in &mut self.level.structures {
            structure.draw(&Vec2::new(0., 0.), &self.level.space).await;

            match &structure.menu {
                Some(menu) => menu.draw().await,
                None => {},
            }
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