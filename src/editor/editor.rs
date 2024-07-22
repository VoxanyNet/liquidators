use std::time::Instant;

use gamelibrary::macroquad_to_rapier;
use liquidators_lib::{level::Level, structure::Structure};
use macroquad::{color::RED, input::{self, is_key_down, is_key_pressed, is_mouse_button_released, mouse_position}, math::{Rect, Vec2}, time::get_fps, window::screen_height};
use gamelibrary::traits::{HasCollider, HasRigidBody};
use nalgebra::vector;
use rapier2d::{dynamics::{RigidBody, RigidBodyBuilder}, geometry::{Collider, ColliderBuilder, ColliderHandle, Group}};

pub struct Editor {
    pub level: Level
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 

impl Editor {

    pub fn spawn_structure(&mut self) {
        if is_key_pressed(input::KeyCode::E) {

            let rigid_body_handle = self.level.space.rigid_body_set.insert(
                RigidBodyBuilder::dynamic()
                    .position(
                        vector![mouse_position().0 - 20., (mouse_position().1 * -1. + screen_height()) - 20.].into()
                    )
            );

            let collider = ColliderBuilder::cuboid(20., 20.)
                .mass(10.)
                .restitution(0.)
                .build();

            let collider_handle = self.level.space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut self.level.space.rigid_body_set);

            let new_structure = Structure { 
                rigid_body_handle: rigid_body_handle,
                collider_handle: collider_handle,
                color: RED,
                menu: None,
                selected: false,
                dragging: false,
                drag_offset: None,
                resize_handles: [Rect::new(0., 0., 0., 0.); 4]
            };
            
            self.level.structures.push(new_structure);

        }

        if is_key_pressed(input::KeyCode::Q) {

            let rigid_body_handle = self.level.space.rigid_body_set.insert(
                RigidBodyBuilder::fixed()
                    .position(
                        vector![mouse_position().0 - 20., (mouse_position().1 * -1. + screen_height()) - 20.].into()
                    )
            );

            let collider = ColliderBuilder::cuboid(20., 20.)
                .mass(10.)
                .restitution(0.)
                .build();

            let collider_handle = self.level.space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut self.level.space.rigid_body_set);

            let new_structure = Structure { 
                rigid_body_handle: rigid_body_handle,
                collider_handle: collider_handle,
                color: RED,
                menu: None,
                selected: false,
                dragging: false,
                drag_offset: None,
                resize_handles: [Rect::new(0., 0., 0., 0.); 4]
            };
            
            self.level.structures.push(new_structure);

        }
    }

    pub fn step_space(&mut self) {
        if is_key_down(input::KeyCode::F) {

            let mut owned_rigid_bodies = vec![];
            let mut owned_colliders = vec![];

            for structure in &self.level.structures {
                owned_rigid_bodies.push(structure.rigid_body_handle);
                owned_colliders.push(structure.collider_handle);
            }
            self.level.space.step(owned_rigid_bodies, owned_colliders);
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

            let rigid_body = self.level.space.rigid_body_set.get(*structure.get_rigid_body_handle()).unwrap();

            //println!("x: {}, y: {}", rigid_body.position().translation.x, rigid_body.position().translation.y);

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

        //macroquad::window::set_fullscreen(true);

        loop { 

            self.tick();

            self.draw().await;

            macroquad::window::next_frame().await;
        }

    }
}