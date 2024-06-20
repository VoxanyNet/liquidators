use gamelibrary::{collider::Collider, proxies::macroquad::{color::colors::RED, math::vec2::Vec2}, rigid_body::RigidBody, translate_coordinates};
use liquidators_lib::{level::Level, structure::Structure};
use macroquad::{input::{self, is_key_down, is_key_pressed, is_mouse_button_released, mouse_position}, window::screen_height};
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
                color: RED,
                menu: None,
                selected: false,
                dragging: false,
                drag_offset: None
            };
            
            self.level.structures.push(new_structure);

        }

        if is_key_pressed(input::KeyCode::Q) {

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
                menu: None,
                selected: false,
                dragging: false,
                drag_offset: None
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

        // tick all Structures
        for structure_index in 0..self.level.structures.len() {
            let mut structure = self.level.structures.remove(structure_index);

            structure.tick_editor(&mut self.level);

            self.level.structures.insert(structure_index, structure);


        }

        self.handle_menus();

    }

    pub fn handle_menus(&mut self) {
        // this needs to be a function on the editor struct because structures cannot delete themselves

        let mut structure_index = 0;
        let mut structures_length = self.level.structures.len();

        loop {

            if structure_index >= structures_length {
                break;
            }

            let structure = self.level.structures.remove(structure_index);

            let result = structure.handle_menu(&mut self.level.space);

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