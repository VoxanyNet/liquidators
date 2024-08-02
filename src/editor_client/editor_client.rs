use std::{fs, time::Instant};

use gamelibrary::{macroquad_to_rapier, menu::Button, mouse_world_pos, space::Space, sync::client::SyncClient};
use liquidators_lib::{level::Level, structure::Structure};
use macroquad::{camera::{self, set_camera, set_default_camera, Camera2D}, color::{DARKGRAY, RED}, input::{self, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down, is_mouse_button_released, mouse_delta_position, mouse_position, mouse_wheel, KeyCode}, math::{vec2, Rect, Vec2}, prelude::camera::mouse::Camera, time::get_fps, window::{screen_height, screen_width}};
use gamelibrary::traits::{HasCollider, HasRigidBody};
use nalgebra::vector;
use rapier2d::{dynamics::{RigidBody, RigidBodyBuilder}, geometry::{Collider, ColliderBuilder, ColliderHandle, Group}};

pub struct EditorClient {
    pub level: Level,
    pub save_button: Button,
    pub load_button: Button,
    pub camera_rect: Rect,
    pub sync_client: SyncClient<Level>
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 

impl EditorClient {

    pub fn connect(url: &str) -> Self {
        let (sync_client, level): (SyncClient<Level>, Level) = SyncClient::connect(url);
        
        let save_button = Button::new(
            "Save".into(),
            Rect { x: 0., y: 0., w: 50., h: 30. },
            DARKGRAY
        );
    
        let load_button = Button::new(
            "Load".into(),
            Rect { x: 0., y: 30., w: 50., h: 30. },
            DARKGRAY
        );

        let camera_rect = Rect::new(0., 0., 1280., 720.);

        Self {
            level,
            save_button,
            load_button,
            camera_rect,
            sync_client
        }
    }

    pub fn spawn_structure(&mut self) {

        if is_key_released(input::KeyCode::E) {

            let mouse_world_pos = mouse_world_pos(&self.camera_rect);

            let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);

            let rigid_body_handle = self.level.space.rigid_body_set.insert(
                RigidBodyBuilder::dynamic()
                    .position(
                        vector![rapier_mouse_world_pos.x, rapier_mouse_world_pos.y].into()
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
                drag_offset: None
            };
            
            self.level.structures.push(new_structure);

        }

        if is_key_pressed(input::KeyCode::Q) {

            let mouse_world_pos = mouse_world_pos(&self.camera_rect);

            let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);

            let rigid_body_handle = self.level.space.rigid_body_set.insert(
                RigidBodyBuilder::fixed()
                    .position(
                        vector![rapier_mouse_world_pos.x, rapier_mouse_world_pos.y].into()
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
                drag_offset: None
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

    pub fn update_camera(&mut self) {
        if mouse_wheel().1 < 0. {
            self.camera_rect.w *= 1.1;
            self.camera_rect.h *= 1.1;
        }

        if mouse_wheel().1 > 0. {

            self.camera_rect.w /= 1.1;
            self.camera_rect.h /= 1.1;
        }

        if is_mouse_button_down(input::MouseButton::Middle) {
            self.camera_rect.x += mouse_delta_position().x * 200.;
            self.camera_rect.y += mouse_delta_position().y * 200.;
        }
    }

    pub fn tick(&mut self) {

        self.update_camera();

        //println!("{:?}", mouse_world_pos(&self.camera));

        //self.zoom_camera();
        // spawn square structure at mouse position
        self.spawn_structure();
        
        self.step_space();     

        self.save_button.update(&self.camera_rect);   
        self.load_button.update(&self.camera_rect);

        // for (handle, collider) in self.level.space.collider_set.iter() {
        //     println!("{}", collider.shape().as_cuboid().unwrap().half_extents);
        // }

        if self.save_button.clicked {
            fs::write("level.bin", bitcode::serialize(&self.level).unwrap()).unwrap();
        }

        if self.load_button.clicked {
            self.level = bitcode::deserialize(
                &fs::read("level.bin").unwrap()
            ).unwrap()
        }
            
        // tick all Structures
        for structure_index in 0..self.level.structures.len() {
            let mut structure = self.level.structures.remove(structure_index);

            structure.tick_editor(&mut self.level, &self.camera_rect);

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

        let mut camera = Camera2D::from_display_rect(self.camera_rect);
        camera.zoom.y = -camera.zoom.y;
        set_camera(
            &camera
        );

        for structure in &mut self.level.structures {
            structure.draw(&Vec2::new(0., 0.), &self.level.space).await;

            match &structure.menu {
                Some(menu) => menu.draw().await,
                None => {},
            }
        }

        set_default_camera();

        self.save_button.draw().await;
        self.load_button.draw().await;

        

    }

    pub async fn run(&mut self) {

        //macroquad::window::set_fullscreen(true);

        loop { 

            self.tick();

            self.draw().await;

            self.sync_client.sync(&mut self.level);

            macroquad::window::next_frame().await;
        }

    }
}