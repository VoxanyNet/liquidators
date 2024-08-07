use std::collections::HashMap;

use gamelibrary::{sync::client::SyncClient, texture_loader::TextureLoader, time::Time};
use liquidators_lib::game_state::GameState;
use macroquad::{color::{colors, Color}, input::{is_key_down, is_key_released}, math::Vec2};
use rand::prelude::SliceRandom;

use rand::thread_rng;

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
            
            self.draw().await;

            self.step_space();

            self.sync_client.sync(&mut self.game_state);
    
            macroquad::window::next_frame().await;


        }
    }

    pub fn step_space(&mut self) {
        let mut owned_rigid_bodies = vec![];
        let mut owned_colliders = vec![];

        // we need a better way of extracting these
        for structure in &self.game_state.level.structures {

            if let Some(owner) = &structure.owner {

                if self.uuid == *owner {
                    owned_rigid_bodies.push(structure.rigid_body_handle);
                    owned_colliders.push(structure.collider_handle);
                }

                else {
                    continue;
                }
                
            };
            
        }
        
        self.game_state.level.space.step(owned_rigid_bodies, owned_colliders); 
    }

 
    pub async fn draw(&mut self) {
        for structure in self.game_state.level.structures.iter_mut() {

            structure.draw(&self.game_state.level.space, &mut self.textures).await;
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

    pub fn tick(&mut self) {

        self.control_camera();

        self.save_state();       

        self.last_tick = Time::now(); 

    }
}