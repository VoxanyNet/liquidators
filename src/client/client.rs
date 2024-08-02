use std::{collections::HashMap, fs, io::Read, time::{Instant, SystemTime, UNIX_EPOCH}};

use gamelibrary::{macroquad_to_rapier, space::SpaceDiff, sync::client::SyncClient, texture_loader::TextureLoader, time::Time};
use diff::Diff;
use liquidators_lib::{game_state::{GameState, GameStateDiff}, physics_square::PhysicsSquare, TickContext};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use macroquad::{color::{colors, Color}, input::{is_key_down, is_key_released, is_mouse_button_released, mouse_position, KeyCode}, math::Vec2, texture::Texture2D, time::get_fps};
use gamelibrary::traits::HasOwner;
use gamelibrary::traits::HasCollider;
use nalgebra::vector;
use rand::prelude::SliceRandom;

use rand::thread_rng;
use rapier2d::{dynamics::{rigid_body, RigidBodyType}, geometry::BroadPhase, parry::shape::Cuboid, prelude::{ColliderBuilder, ColliderHandle, QueryFilter}};

// Return a random color
pub fn random_color() -> Color {

    let colors = [colors::RED, colors::BLUE, colors::GREEN];

    let mut rng = thread_rng();

    colors.choose(&mut rng).unwrap().clone()
}

pub struct Client {
    pub game_state: GameState,
    pub is_host: bool,
    pub textures: TextureLoader,
    pub sounds: HashMap<String, macroquad::audio::Sound>,
    pub last_tick: Time,
    pub uuid: String,
    pub camera_offset: Vec2,
    pub update_count: i32,
    pub start_time: Time,
    pub square_color: Color,
    pub sync_client: SyncClient<GameState>
}

impl Client {

    pub async fn run(&mut self) {

        loop {
    
            self.tick();

            let mut owned_rigid_bodies = vec![];
            let mut owned_colliders = vec![];

            // we need a better way of extracting these
            for physics_square in &self.game_state.physics_squares {

                if physics_square.owner != self.uuid {
                    continue;
                }

                owned_rigid_bodies.push(physics_square.rigid_body_handle);
                owned_colliders.push(physics_square.collider_handle);
            }
            
            self.game_state.space.step(owned_rigid_bodies, owned_colliders);   
            
            self.draw().await;

            let then = Instant::now();

            self.sync_client.sync(&mut self.game_state);

            println!("{:?}", then.elapsed());
    
            macroquad::window::next_frame().await;
    
            // cap framerate at 200fps (or 5 ms per frame)
            // TODO: this needs to take into account the time it took to draw the last frame
            // std::thread::sleep(
            //     Duration::from_millis(5)
            // );



        }
    }

 
    pub async fn draw(&mut self) {
        for entity in self.game_state.physics_squares.iter_mut() {

            entity.draw(&self.camera_offset, &self.game_state.space).await;
        }
    }

    pub fn connect(url: &str) -> Self {

        let uuid = gamelibrary::uuid();

        println!("{}", uuid);
        
        let (sync_client, game_state): (SyncClient<GameState>, GameState) = SyncClient::connect(url);
        
        Self {
            game_state,
            is_host: true,
            textures: TextureLoader::new(),
            sounds: HashMap::new(),
            last_tick: Time::now(),
            uuid,
            camera_offset: Vec2::new(0., 0.),
            update_count: 0,
            start_time: Time::now(),
            square_color: random_color(),
            sync_client
        }
    }


    pub fn connect_as_master() {

    }

    pub fn control_camera(&mut self) {
        if is_key_down(macroquad::input::KeyCode::Right) {
            self.camera_offset.x += 1.0 * self.last_tick.elapsed().num_milliseconds() as f32;
        }

        if is_key_down(macroquad::input::KeyCode::Left) {
            self.camera_offset.x -= 1.0 * self.last_tick.elapsed().num_milliseconds() as f32;
        }

        if is_key_down(macroquad::input::KeyCode::Down) {
            self.camera_offset.y -= 1.0 * self.last_tick.elapsed().num_milliseconds() as f32;
        }

        if is_key_down(macroquad::input::KeyCode::Up) {
            self.camera_offset.y += 1.0 * self.last_tick.elapsed().num_milliseconds() as f32;
        }
    }

    fn save_state(&mut self) {
        if is_key_released(macroquad::input::KeyCode::F5) {

            let game_state_binary = bitcode::serialize(&self.game_state).unwrap();

            std::fs::write("state.bin", game_state_binary).unwrap();
        }

        if is_key_released(macroquad::input::KeyCode::F6) {
            self.game_state = bitcode::deserialize(
                &std::fs::read("state.bin").expect("failed to read state file")
            ).expect("failed to deserialize state file");
        }
    }

    fn spawn_physics_square(&mut self) {
        if is_mouse_button_released(macroquad::input::MouseButton::Left) {

            let mouse_pos = mouse_position();

            let rapier_mouse_pos = macroquad_to_rapier(&Vec2::new(mouse_pos.0, mouse_pos.1));

            self.game_state.physics_squares.push( 
                PhysicsSquare::new(
                    &mut self.game_state.space,
                    Vec2::new(rapier_mouse_pos.x, rapier_mouse_pos.y),
                    RigidBodyType::Dynamic,
                    20., 
                    20., 
                    &self.uuid,
                    false,
                    self.square_color
                )
            );
        }
    }

    pub fn tick(&mut self) {

        self.control_camera();

        self.save_state();       

        self.spawn_physics_square();

        for index in 0..self.game_state.physics_squares.len() {

            let mut entity = self.game_state.physics_squares.remove(index);

            let mut tick_context = TickContext {
                game_state: &mut self.game_state,
                is_host: &mut self.is_host,
                textures: &mut self.textures,
                sounds: &mut self.sounds,
                time: &self.last_tick,
                uuid: &self.uuid,
                camera_offset: &mut self.camera_offset,
            };

            // we only tick the entity if we own it
            if entity.get_owner() == *self.uuid {
                entity.tick(&mut tick_context);
            }
            
            // put the entity back in the same index so it doesnt FUCK things up
            self.game_state.physics_squares.insert(index, entity)

        }

        self.last_tick = Time::now(); 

    }
}