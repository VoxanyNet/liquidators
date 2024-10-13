use std::{fs, time::{Duration, Instant}};

use gamelibrary::{sync::client::SyncClient, texture_loader::TextureLoader, time::Time, traits::HasPhysics};
use gilrs::{Event, GamepadId, Gilrs};
use liquidators_lib::{game_state::GameState, level::Level, player::Player, vec_remove_iter::IntoVecRemoveIter, TickContext};
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{colors, Color, WHITE}, input::{self, is_key_released, is_mouse_button_down, is_quit_requested, mouse_delta_position, mouse_position, mouse_wheel, prevent_quit, KeyCode}, math::{vec2, Rect, Vec2}, miniquad::window::{set_mouse_cursor, set_window_size}, prelude::{camera::mouse, debug}, text::draw_text, texture::{draw_texture, draw_texture_ex, DrawTextureParams}, time::get_fps, window::screen_width};
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
    pub last_tick: Instant,
    pub uuid: String,
    pub camera_offset: Vec2,
    pub update_count: i32,
    pub start_time: Time,
    pub square_color: Color,
    pub sync_client: SyncClient<GameState>,
    pub last_sync: Instant,
    pub camera_rect: Rect,
    pub gilrs: Gilrs,
    pub active_gamepad: Option<GamepadId>
}

impl Client {

    pub fn tick(&mut self) {

        if is_key_released(KeyCode::R) || self.active_gamepad.map_or(false, |active_gamepad| {self.gilrs.gamepad(active_gamepad).is_pressed(gilrs::Button::Start)}){
            self.reset_level();
        }

        let mut tick_context = TickContext {
            is_host: &mut self.is_host,
            textures: &mut self.textures,
            uuid: &self.uuid,
            camera_offset: &mut self.camera_offset,
            last_tick: &self.last_tick,
            camera_rect: &self.camera_rect,
            active_gamepad: &self.active_gamepad,
            gilrs: &mut self.gilrs
        };

        

        self.game_state.tick(
            &mut tick_context
        );
        
        self.update_camera();

        if is_key_released(KeyCode::B) {
            self.game_state.level = Level::from_save("level.bin".to_string());
        }

        self.gilrs.next_event();

        self.save_state();

        self.last_tick = Instant::now();

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

    pub fn disconnect(&mut self) {

       let mut players = self.game_state.level.players.into_vec_remove_iter();

       while let Some(mut item) = players.next() {

            if item.element.owner != self.uuid {
                item.restore();
            }

            else {
                item.element.remove_body_and_collider(&mut self.game_state.level.space);
            }
        }


        // send a final sync to the server
        self.sync_client.sync(&mut self.game_state);
    }

    pub fn reset_level(&mut self) {
        println!("resetting");
        let reset_level: Level = bitcode::deserialize(&fs::read("level.bin").unwrap()).unwrap();

        self.game_state.level = reset_level;

        for structure in self.game_state.level.structures.iter_mut() {
            structure.owner = Some(self.uuid.clone())
        }

        for brick in self.game_state.level.bricks.iter_mut() {
            brick.owner = Some(self.uuid.clone());
        }

        Player::spawn(&mut self.game_state.level.players, &mut self.game_state.level.space, self.uuid.clone(), &vec2(100., 300.));

    }

    pub async fn run(&mut self) {

        //prevent_quit();

        loop {

            if is_quit_requested() {
                self.disconnect();

                break;
            }

            self.tick(); 

            if is_key_released(KeyCode::H) {
                println!("paused");
            }
            
            self.draw().await;

            // only sync 30 tps
            // this could probably be optimized but this is more readable
            if self.last_sync.elapsed().as_secs_f32() > 1./60. {

                let then = Instant::now();
                self.sync_client.sync(&mut self.game_state);

                let elapsed = then.elapsed();
                println!("{}", elapsed.as_micros());
                self.last_sync = Instant::now();
            }            

            // calculate the time we need to sleep to lock the framerate at 120
            //let sleep_duration = Duration::from_millis(7).saturating_sub(self.last_tick.elapsed());
            

            //std::thread::sleep(sleep_duration);
    
        }
    }

 
    pub async fn draw(&mut self) {

        
    
        draw_text(format!("fps: {}", get_fps()).as_str(), screen_width() - 120., 25., 30., WHITE);
        
        let mut camera = Camera2D::from_display_rect(self.camera_rect);
        camera.zoom.y = -camera.zoom.y;

        set_camera(
            &camera
        );

    
        self.game_state.draw(&mut self.textures).await;

        set_default_camera();

        macroquad::window::next_frame().await;
    }

    pub fn connect(url: &str) -> Self {

        debug!("test");



        let mut camera_rect = Rect::new(0., 300., 1280., 720.);
        
        let uuid = gamelibrary::uuid();

        println!("{}", uuid);
        
        let (sync_client, mut game_state): (SyncClient<GameState>, GameState) = SyncClient::connect(url);

        //let mut game_state: GameState = bitcode::deserialize(&fs::read("level.bin").unwrap()).unwrap();

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"CEXected!".into());
       

        // if we are the first player to join, we take ownership of everything
        if game_state.level.players.len() == 0 {
            for structure in game_state.level.structures.iter_mut() {
                structure.owner = Some(uuid.clone())
            }

            for brick in game_state.level.bricks.iter_mut() {
                brick.owner = Some(uuid.clone());
            }
        }

        Player::spawn(&mut game_state.level.players, &mut game_state.level.space, uuid.clone(), &vec2(100., 300.));

        let gilrs = Gilrs::new().unwrap();

        let mut active_gamepad: Option<GamepadId> = None; 

        active_gamepad = gilrs.gamepads().next().map_or(None, |gamepad|{Some(gamepad.0)});
        
        Self {
            game_state,
            is_host: true,
            textures: TextureLoader::new(), 
            last_tick: Instant::now(),
            uuid,
            camera_offset: Vec2::new(0., 0.),
            update_count: 0,
            start_time: Time::now(),
            square_color: random_color(),
            sync_client,
            last_sync: Instant::now(),
            camera_rect,
            gilrs,
            active_gamepad
        }
    }


    pub fn connect_as_master() {

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