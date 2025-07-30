use std::{fs, time::{Duration, Instant}};

use gamelibrary::{arenaiter::SyncArenaIterator, log, menu::Button, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, space::{Space, SyncRigidBodyHandle}, swapiter::SwapIter, sync::client::SyncClient, sync_arena::SyncArena, texture_loader::TextureLoader, uuid_string};
use nalgebra::vector;
use parry2d::math::{Isometry, Vector};
use rapier2d::prelude::{ColliderBuilder, Cuboid, FixedJoint, FixedJointBuilder, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
use crate::{level::Level, structure::{self, Structure}, TickContext};
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{DARKGRAY, WHITE}, input::{self, is_key_down, is_key_released, is_mouse_button_down, mouse_delta_position, mouse_position, mouse_wheel, KeyCode}, math::{Rect, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}, text::draw_text, time::get_fps, window::{screen_height, screen_width}};
use gamelibrary::traits::HasPhysics;

pub struct EditorClient {
    pub uuid: String,
    pub level: Level,
    pub save_button: Button,
    pub load_button: Button,
    pub camera_rect: Rect,
    pub sync_client: SyncClient<Level>,
    pub textures: TextureLoader,
    pub last_tick: web_time::Instant,
    pub enable_physics: bool,
    pub last_tick_duration: Duration,
    pub selected_physics_objects: Vec<RigidBodyHandle>, // order matters!
    pub first_point: Option<Vec2>,
    pub second_point: Option<Vec2>,
    pub released: bool,
    clipboard: SyncArena<Structure>, // we need to make this generic somehow
    copy_mouse_position: Vec2, // the position of the mouse when we copied the structures
    pasted: bool
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 

impl EditorClient {

    pub async fn connect(url: &str) -> Self {

        let uuid = uuid_string();

        let (sync_client, level): (SyncClient<Level>, Level) = SyncClient::connect(url).await;
        
        let save_button = Button::new(
            "Save".into(),
            Rect { x: 0., y: 0., w: 50., h: 30. },
            DARKGRAY,
            None,
            None,
            20,
            "assets/fonts/CutePixel.ttf".to_string()
        ).await;
    
        let load_button = Button::new(
            "Load".into(),
            Rect { x: 0., y: 30., w: 50., h: 30. },
            DARKGRAY,
            None,
            None,
            20,
            "assets/fonts/CutePixel.ttf".to_string()
        ).await;

        let camera_rect = Rect::new(0., 200., screen_width() / 1.50, screen_height() / 1.5);

        Self {
            uuid,
            level,
            save_button,
            load_button,
            camera_rect,
            sync_client,
            textures: TextureLoader::new(),
            last_tick: web_time::Instant::now(),
            enable_physics: false,
            last_tick_duration: web_time::Duration::from_nanos(500),
            selected_physics_objects: Vec::new(),
            first_point: None,
            second_point: None,
            released: true,
            clipboard: SyncArena::default(),
            copy_mouse_position: Vec2::ZERO,
            pasted: false

        }
    }

    pub fn toggle_physics(&mut self) {

        if is_key_released(input::KeyCode::F) {
            self.enable_physics = !self.enable_physics;
        }
    }

    pub fn step_space(&mut self) {

        if self.enable_physics {

            let mut owned_rigid_bodies = vec![];
            let mut owned_colliders = vec![];
            let mut owned_joints = vec![];

            for shotgun in &self.level.shotguns {
                owned_colliders.push(shotgun.collider());
                owned_rigid_bodies.push(shotgun.rigid_body())
            }

            for (_, structure) in &self.level.structures {
                owned_rigid_bodies.push(structure.rigid_body_handle);
                owned_colliders.push(structure.collider_handle);
            }

            for brick in &self.level.bricks {
                owned_rigid_bodies.push(brick.rigid_body_handle().clone());
                owned_colliders.push(brick.collider_handle().clone());
            }

            self.level.space.step(&owned_rigid_bodies, &owned_colliders, &mut owned_joints, self.last_tick_duration);
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

        let camera_speed = match is_key_down(input::KeyCode::LeftShift) {
            true => 10.,
            false => 5.,
        };
        if is_key_down(input::KeyCode::W) {
            self.camera_rect.y -= camera_speed;
        }

        if is_key_down(input::KeyCode::S) {
            self.camera_rect.y += camera_speed;
        }
        
        if is_key_down(input::KeyCode::A) {
            self.camera_rect.x -= camera_speed;
        }

        if is_key_down(input::KeyCode::D) {
            self.camera_rect.x += camera_speed;
        }
        if is_mouse_button_down(input::MouseButton::Middle) {
            self.camera_rect.x += mouse_delta_position().x * 200.;
            self.camera_rect.y += mouse_delta_position().y * 200.;
        }
    }

    fn handle_buttons(&mut self) {
        self.save_button.update(Some(&self.camera_rect));   
        self.load_button.update(Some(&self.camera_rect));

        

        if self.save_button.clicked {
            fs::write("level.yaml", serde_yaml::to_string(&self.level).unwrap()).unwrap();
        }

        if self.load_button.clicked {
            self.level = serde_yaml::from_slice(
                &fs::read("level.yaml").unwrap()
            ).unwrap()
        }
    }

    pub fn tick(&mut self) {

        //dbg!(&self.selected_physics_objects);
        self.delete_selected_structures();
        self.create_drag_select();
        self.select_physics_objects();
        self.level.editor_tick(&self.camera_rect, &self.uuid, &mut self.textures);

        self.copy_selected_structures();
        self.paste_copied_structures();

        self.joint_selected_bodies();

        self.toggle_physics();

        self.update_camera();

        self.handle_buttons();
        
        self.handle_menus();

        self.step_space();   


        self.last_tick_duration = self.last_tick.elapsed();
        self.last_tick = web_time::Instant::now();         

    }

    pub fn joint_selected_bodies(&mut self) {

        if !is_key_released(KeyCode::J) {
            return;
        }

        for index in 0..self.selected_physics_objects.len() {

            // last selected body, nothing to joint to
            if index == self.selected_physics_objects.len() - 1 {
                continue;
            }

            let body_handle = self.selected_physics_objects[index];
            let sync_body_handle = self.level.space.sync_rigid_body_set.get_sync_handle(body_handle);

            let body_position = self.level.space.sync_rigid_body_set.get_local(body_handle).unwrap().translation().clone();

            let next_body_handle = self.selected_physics_objects[index + 1];
            let next_body_position = self.level.space.sync_rigid_body_set.get_local(next_body_handle).unwrap().translation().clone();

            // maybe change this to revolute joint for more interesting physics
            let joint = FixedJointBuilder::new()
                .local_anchor1(vector![0., 0.].into())
                .local_anchor2(
                    vector![
                        body_position.x - next_body_position.x,
                        body_position.y - next_body_position.y
                    ].into()
                )
                .contacts_enabled(false);

            let joint_handle = self.level.space.sync_impulse_joint_set.insert_sync(body_handle, next_body_handle, joint, true);

            for (_, structure) in &mut self.level.structures {
                if *structure.rigid_body_handle() == sync_body_handle {
                    structure.joint_handle = Some(joint_handle);
                }
            }
            
        }


    }
    // use the drag selection to select physics objects
    pub fn select_physics_objects(&mut self) {

        let (first_point, second_point) = match (self.first_point, self.second_point) {
            
            (Some(first_point), Some(second_point)) => {
                (first_point, second_point)
            },
            _ => {
                self.selected_physics_objects.clear();
                return;
            }
        };

        let half_extents = Vec2::new((second_point.x - first_point.x).abs() / 2., (second_point.y - first_point.y).abs() / 2.);

        let mut selections = Vec::new();

        self.level.space.query_pipeline.intersections_with_shape(
            &self.level.space.sync_rigid_body_set.rigid_body_set, 
            &self.level.space.sync_collider_set.collider_set, 
            &Isometry::new(vector![first_point.x + half_extents.x, first_point.y - half_extents.y].into(), 0.), 
            &Cuboid::new(
                Vector::new(
                    half_extents.x, 
                    half_extents.y
                )
            ), 
            QueryFilter::default(), 
            |collider| {
                

                let body_handle = self.level.space.sync_collider_set.get_local(collider).unwrap().parent().unwrap();

                selections.push(body_handle);
                true
            }
        );

        for selection in &selections {

            if !self.selected_physics_objects.contains(&selection) {
                self.selected_physics_objects.push(selection.clone());
            }
        }

        self.selected_physics_objects.retain(|&selection| selections.contains(&selection));


    }

    pub fn highlight_selected_physics_objects(&self) {
        for (_, structure) in &self.level.structures {
            
            let local_handle = self.level.space.sync_rigid_body_set.get_local_handle(structure.rigid_body_handle);
            if self.selected_physics_objects.contains(&local_handle) {
                structure.draw_outline(&self.level.space, 1.);
            }
        }
    }

    pub fn delete_selected_structures(&mut self) {

        if !is_key_released(KeyCode::Delete) {
            return;
        }

        let mut structure_iter = SyncArenaIterator::new(&mut self.level.structures);

        while let Some((structure, structures)) = structure_iter.next() {

            if self.selected_physics_objects.contains(
                &self.level.space.sync_rigid_body_set.get_local_handle(structure.rigid_body_handle)
            ) {
                structure.despawn(&mut self.level.space);
            } else {
                structure_iter.restore(structure);
            }
        }
    }

    pub fn copy_selected_structures(&mut self) {


        if !(is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::C)) {
            return;
        }

        self.copy_mouse_position = self.first_point.unwrap();

        let mut copied_structures = SyncArena::new();

        for (_, structure) in &self.level.structures {
            if self.selected_physics_objects.contains(
                &self.level.space.sync_rigid_body_set.get_local_handle(structure.rigid_body_handle)
            ) {

                copied_structures.insert(structure.clone());
            }
        }

        self.clipboard = copied_structures;
    }

    pub fn paste_copied_structures(&mut self) {

        if !(is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::V)) {
            self.pasted = false;
            return;
        }

        if self.pasted == true {
            return;
        }

        self.pasted = true;

        let copy_offset = rapier_mouse_world_pos(&self.camera_rect) - self.copy_mouse_position;

        let mut pasted_structures = self.clipboard.clone();

        for (_, structure) in &mut pasted_structures {

            let structure_body = self.level.space.sync_rigid_body_set.get_sync(structure.rigid_body_handle).unwrap();
            let structure_collider = self.level.space.sync_collider_set.get_sync(structure.collider_handle).unwrap();

            let new_position = vector![
                structure_body.translation().x + copy_offset.x,
                structure_body.translation().y + copy_offset.y
            ];

            // we need to create a new body and collider for the copied structure

            structure.rigid_body_handle = self.level.space.sync_rigid_body_set.insert_sync(
                RigidBodyBuilder::dynamic()
                    .additional_mass(structure_body.mass())
                    .position(new_position.into())
                    .rotation(structure_body.rotation().angle())
            );

            let half_extents = structure_collider.shape().as_cuboid().unwrap().half_extents;

            structure.collider_handle = self.level.space.sync_collider_set.insert_with_parent_sync(
                ColliderBuilder::cuboid(half_extents.x, half_extents.y)
                    .mass(structure_collider.mass())
                    .rotation(structure_collider.rotation().angle()),
                structure.rigid_body_handle,
                &mut self.level.space.sync_rigid_body_set
            );
        }

        for structure in pasted_structures {
            self.level.structures.insert(structure);
        }
    }

    pub fn create_drag_select(&mut self) {

        if is_key_down(input::KeyCode::Escape) {
            self.first_point = None;
            self.second_point = None;
        }

        if !is_key_down(KeyCode::LeftShift) {
            return;
        }


        
        if is_mouse_button_down(input::MouseButton::Left) && self.released == true {
            self.first_point = Some(rapier_mouse_world_pos(&self.camera_rect));
            
        }

        if is_mouse_button_down(input::MouseButton::Left) && self.first_point.is_some() {
            self.second_point = Some(rapier_mouse_world_pos(&self.camera_rect));

            self.released = false;

        }

        if !is_mouse_button_down(input::MouseButton::Left) {
            self.released = true;
        }

        

        
        

    }

    pub fn handle_menus(&mut self) {
        // this needs to be a function on the editor struct because structures cannot delete themselves

        let mut structurs_iter = SyncArenaIterator::new(&mut self.level.structures);

        while let Some((structure, structures)) = structurs_iter.next() {


            if structure.editor_owner != self.uuid {
                structurs_iter.restore(structure);

                continue;
            }

            let result = structure.handle_menu(&mut self.level.space);

            if let Some(structure) = result {
                structurs_iter.restore(structure);
            }
            
        }
    
            
    }

    pub async fn draw(&mut self) {

        let mut camera = Camera2D::from_display_rect(self.camera_rect);
        camera.zoom.y = -camera.zoom.y;
        set_camera(
            &camera
        );
        
        self.level.editor_draw(&mut self.textures, &self.camera_rect).await;
        self.draw_selection();

        set_default_camera();

        draw_text(format!("fps: {}", get_fps()).as_str(), screen_width() - 120., 25., 30., WHITE);

        self.save_button.draw().await;
        self.load_button.draw().await;

        if is_key_down(KeyCode::LeftAlt) {
            let macroquad_screen_mouse_pos = mouse_position();
            let macroquad_world_mouse_pos = mouse_world_pos(&self.camera_rect);
            let rapier_world_mouse_pos = rapier_mouse_world_pos(&self.camera_rect);

            draw_text(format!("screen: {}, {}", macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1), macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1 + 15., 20., WHITE);
            draw_text(format!("macroquad world: {}, {}", macroquad_world_mouse_pos.x, macroquad_world_mouse_pos.y ), macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1 + 30., 20., WHITE);
            draw_text(format!("rapier world: {}, {}", rapier_world_mouse_pos.x, rapier_world_mouse_pos.y), macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1 + 45., 20., WHITE);
        }
        
        

    }

    pub fn draw_selection(&self) {
        match self.first_point {
            Some(first_point) => {
                
                match self.second_point {
                    Some(second_point) => {

                        

                        let first_point = rapier_to_macroquad(&first_point);
                        let second_point = rapier_to_macroquad(&second_point);

                        

                        draw_rectangle_lines(first_point.x, first_point.y, second_point.x - first_point.x, second_point.y - first_point.y, 5., WHITE);
                    },
                    None => {
                        let first_point = rapier_to_macroquad(&first_point);
                        let second_point = rapier_to_macroquad(&rapier_mouse_world_pos(&self.camera_rect));

                        draw_rectangle_lines(first_point.x, first_point.y, second_point.x - first_point.x, second_point.y - first_point.y, 5., WHITE);
                    },
                }
            },
            None => {},
        }
    }

    pub async fn run(&mut self) {

        // let mut sound = Sound::new("assets/sounds/radio.mp3").unwrap();
        // sound.set_volume(0.25);

        // sound.play();

        //macroquad::window::set_fullscreen(true);

        loop { 

            self.tick();

            self.draw().await;

            self.sync_client.sync(&mut self.level);

            macroquad::window::next_frame().await;
        }

    }
}