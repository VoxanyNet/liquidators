use std::{collections::HashMap, thread::sleep, time::{Duration, Instant}};

use ears::{AudioController, Sound};
use gamelibrary::{sync::client::SyncClient, texture_loader::TextureLoader, time::Time};
use liquidators_lib::{game_state::GameState, level::Level, player::Player, TickContext};
use macroquad::{color::{colors, Color, WHITE}, input::{is_key_down, is_key_released, KeyCode}, math::{vec2, Vec2}, text::draw_text, time::get_fps, window::screen_width};
use rand::prelude::SliceRandom;
use gamelibrary::traits::HasPhysics;

use rand::thread_rng;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

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
    pub last_tick: Instant,
    pub uuid: String,
    pub camera_offset: Vec2,
    pub update_count: i32,
    pub start_time: Time,
    pub square_color: Color,
    pub sync_client: SyncClient<GameState>,
    pub last_sync: Instant
}

impl Client {

    pub fn tick(&mut self) {

        let mut tick_context = TickContext {
            is_host: &mut self.is_host,
            textures: &mut self.textures,
            sounds: &mut self.sounds,
            uuid: &self.uuid,
            camera_offset: &mut self.camera_offset,
            last_tick: &self.last_tick
        };

        self.game_state.tick(
            &mut tick_context
        );

        if is_key_released(KeyCode::B) {
            self.game_state.level = Level::from_save("level.bin".to_string());
        }

        self.control_camera();

        self.save_state();

        self.last_tick = Instant::now();

    }

    pub async fn run(&mut self) {

        loop {
    
            self.tick(); 

            if is_key_released(KeyCode::H) {
                println!("paused");
            }

            println!("dt: {}", self.last_tick.elapsed().as_secs_f32());
            
            self.draw().await;

            // only sync 30 tps
            // this could probably be optimized but this is more readable
            if self.last_sync.elapsed().as_secs_f32() > 1./30. {
                self.sync_client.sync(&mut self.game_state);

                self.last_sync = Instant::now();
            }            

            // calculate the time we need to sleep to lock the framerate at 120
            let sleep_duration = Duration::from_millis(7).saturating_sub(self.last_tick.elapsed());
            

            std::thread::sleep(sleep_duration);
    
        }
    }

 
    pub async fn draw(&mut self) {

        draw_text(format!("fps: {}", get_fps()).as_str(), screen_width() - 120., 25., 30., WHITE);
        
        self.game_state.draw(&mut self.textures).await;

        macroquad::window::next_frame().await;
    }

    pub fn connect(url: &str) -> Self {

        let uuid = gamelibrary::uuid();

        println!("{}", uuid);
        
        let (sync_client, mut game_state): (SyncClient<GameState>, GameState) = SyncClient::connect(url);

        // if we are the first player to join, we take ownership of everything
        if game_state.level.players.len() == 0 {
            for structure in game_state.level.structures.iter_mut() {
                structure.owner = Some(uuid.clone())
            }
        }

        Player::spawn(&mut game_state.level.players, &mut game_state.level.space, uuid.clone(), &vec2(100., 300.));
        
        Self {
            game_state,
            is_host: true,
            textures: TextureLoader::new(),
            sounds: HashMap::new(),
            last_tick: Instant::now(),
            uuid,
            camera_offset: Vec2::new(0., 0.),
            update_count: 0,
            start_time: Time::now(),
            square_color: random_color(),
            sync_client,
            last_sync: Instant::now()
        }
    }


    pub fn connect_as_master() {

    }

    pub fn control_camera(&mut self) {
        if is_key_down(macroquad::input::KeyCode::Right) {
            self.camera_offset.x += 1.0 * self.last_tick.elapsed().as_millis() as f32;
        }

        if is_key_down(macroquad::input::KeyCode::Left) {
            self.camera_offset.x -= 1.0 * self.last_tick.elapsed().as_millis() as f32;
        }

        if is_key_down(macroquad::input::KeyCode::Down) {
            self.camera_offset.y -= 1.0 * self.last_tick.elapsed().as_millis() as f32;
        }

        if is_key_down(macroquad::input::KeyCode::Up) {
            self.camera_offset.y += 1.0 * self.last_tick.elapsed().as_millis() as f32;
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

    
}