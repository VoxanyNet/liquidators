use std::{fs, sync::{mpsc, Arc, Mutex}, time::Instant};

use gamelibrary::{animation_loader::AnimationLoader, arenaiter::SyncArenaIterator, log, sound::soundmanager::SoundManager, sync::client::SyncClient, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::GamepadId;
use liquidators_lib::{console::Console, game_state::GameState, level::Level, player::player::Player, vec_remove_iter::IntoVecRemoveIter, TickContext};
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::WHITE, input::{self, is_key_released, is_mouse_button_down, is_quit_requested, mouse_delta_position, mouse_wheel, prevent_quit, KeyCode}, math::{vec2, Rect, Vec2}, text::draw_text, time::get_fps, window::{request_new_screen_size, screen_width}};

pub struct Client<S: SoundManager> {
    pub game_state: GameState,
    pub is_host: bool,
    pub textures: TextureLoader,
    pub animations: AnimationLoader,
    pub last_tick: Instant,
    pub uuid: String,
    pub camera_offset: Vec2,
    pub update_count: i32,
    pub sync_client: SyncClient<GameState>,
    pub last_sync: Instant,
    pub camera_rect: Rect,
    pub active_gamepad: Option<GamepadId>,
    pub console: Console,
    pub sounds: S
}

impl<S: SoundManager> Client<S> {
    
    pub fn tick(&mut self) {

        self.console.tick();

        if is_key_released(KeyCode::Tab) {
            self.console.enabled = !self.console.enabled
        }

        if is_key_released(KeyCode::K) {
            request_new_screen_size(886., 480.);
        }

        let mut tick_context = TickContext {
            is_host: &mut self.is_host,
            textures: &mut self.textures,
            uuid: &self.uuid,
            camera_offset: &mut self.camera_offset,
            last_tick: &self.last_tick,
            camera_rect: &self.camera_rect,
            active_gamepad: &self.active_gamepad,
            console: &mut self.console,
            owned_rigid_bodies: &mut vec![],
            owned_colliders: &mut vec![],
            sounds: &mut self.sounds
        };


        self.game_state.tick(
            &mut tick_context
        );

        if is_key_released(KeyCode::R) {
            self.reset_level();
        }
        
        self.update_camera();

        // if is_key_released(KeyCode::B) {
        //     self.game_state.level = Level::from_save("level.yaml".to_string());
        // }

        

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

        
       let mut players_iter = SyncArenaIterator::new(&mut self.game_state.level.players);

       while let Some((mut player, players)) = players_iter.next() {

            if player.owner != self.uuid {
                players_iter.restore(player);
            }

            else {
                player.remove_body_and_collider(&mut self.game_state.level.space);
            }
        }


        // send a final sync to the server
        self.sync_client.sync(&mut self.game_state);
    }

    pub fn reset_level(&mut self) {
        log("resetting");
        let reset_level: Level = bitcode::deserialize(&fs::read("level.bin").unwrap()).unwrap();

        self.game_state.level = reset_level;

        for structure in self.game_state.level.structures.iter_mut() {
            structure.owner = Some(self.uuid.clone())
        }

        for brick in self.game_state.level.bricks.iter_mut() {
            brick.owner = Some(self.uuid.clone());
        }

        Player::spawn(&mut self.game_state.level.players, &mut self.game_state.level.space, self.uuid.clone(), &vec2(100., 300.), &mut self.textures);

    }

    pub fn tick_loop(client: Arc<Mutex<Self>>) {

        Self::tick(&mut *client.lock().unwrap());
    }

    pub fn draw_loop(rx: mpsc::Receiver<GameState>) {
        
    }

    pub async fn run(&mut self) {

        prevent_quit();

        loop {

            if is_quit_requested() {
                self.disconnect();

                break;
            }

            //let then = Instant::now();

            

            // only tick maximum 120 times per second to avoid glitchyness
            if self.last_tick.elapsed().as_millis() >= 8 {

                //println!("{}", self.last_tick.elapsed().as_millis() - 8);

                self.tick();
                

                self.draw().await;
                
                // self.tick() updates self.last_tick automatically unlike self.last_sync
            }
            
            // only sync 30 tps
            // this could probably be optimized but this is more readable
            if self.last_sync.elapsed().as_secs_f32() > 1./120. {

                self.sync_client.sync(&mut self.game_state);

                self.last_sync = Instant::now();

            }   

            //println!("FPS: {}", 1. / then.elapsed().as_secs_f64());
            
            if is_key_released(KeyCode::H) {
                log("paused");
            }
        

                     

            // calculate the time we need to sleep to lock the framerate at 120
            //let sleep_duration = Duration::from_millis(7).saturating_sub(self.last_tick.elapsed());
            

            //std::thread::sleep(sleep_duration);
    
        }
    }

 
    pub async fn draw(&mut self) {
    
        draw_text(format!("fps: {}", get_fps()).as_str(), screen_width() - 120., 25., 30., WHITE);
        //draw_text(format!("uuid: {}", self.uuid).as_str(), screen_width() - 120., 25., 30., WHITE);
        
        let mut camera = Camera2D::from_display_rect(self.camera_rect);
        camera.zoom.y = -camera.zoom.y;

        set_camera(
            &camera
        );

    
        self.game_state.draw(&mut self.textures, &self.camera_rect).await;

        set_default_camera();

        self.console.draw().await;

        macroquad::window::next_frame().await;
    }

    pub async fn connect(url: &str) -> Self {


        let mut textures = TextureLoader::new();

        let camera_rect = Rect::new(0., 200., 1280., 720.);
        
        let uuid = gamelibrary::uuid();

        log(format!("{}", uuid).as_str());
        
        let (sync_client, mut game_state): (SyncClient<GameState>, GameState) = SyncClient::connect(url).await;
    
        let rigid_body_set = &mut game_state.level.space.rigid_body_set;
        // we will start allocating OUR rigid bodies starting at the end of the current set
        let new_free_list_head = rigid_body_set.bodies.capacity();
        // reserve 500 entries in the rigid body and collider sets
        rigid_body_set.bodies.reserve(500);
        // this is only client side
        rigid_body_set.bodies.set_free_list_head(new_free_list_head as u32);

        // do the same for collider set
        let collider_set = &mut game_state.level.space.collider_set;
        let new_free_list_head = collider_set.colliders.capacity();
        collider_set.colliders.reserve(500);
        collider_set.colliders.set_free_list_head(new_free_list_head as u32);

        // if we are the first player to join, we take ownership of everything
        if game_state.level.players.len() == 0 {
            for structure in game_state.level.structures.iter_mut() {
                structure.owner = Some(uuid.clone())
            }

            for brick in game_state.level.bricks.iter_mut() {
                brick.owner = Some(uuid.clone());
            }
            
            for shotgun in game_state.level.shotguns.iter_mut() {
                shotgun.owner = uuid.clone();
            }
        }

        Player::spawn(&mut game_state.level.players, &mut game_state.level.space, uuid.clone(), &vec2(100., 300.), &mut textures);

        //let gilrs = Gilrs::new().unwrap();

        let active_gamepad: Option<GamepadId> = None; 

        // active_gamepad = gilrs.gamepads().next().map_or(None, |gamepad|{Some(gamepad.0)});
        
        Self {
            game_state,
            is_host: true,
            textures, 
            animations: AnimationLoader::new(),
            last_tick: Instant::now(),
            uuid,
            camera_offset: Vec2::new(0., 0.),
            update_count: 0,
            last_sync: Instant::now(),
            camera_rect,
            active_gamepad,
            sync_client,
            console: Console::new(),
            sounds: S::new()
        }
    }


    pub fn connect_as_master() {

    }

    fn save_state(&mut self) {
        if is_key_released(macroquad::input::KeyCode::F5) {

            let game_state_json = serde_json::to_string_pretty(&self.game_state).unwrap();

            std::fs::write("state.json", game_state_json).unwrap();
        }

        if is_key_released(macroquad::input::KeyCode::F6) {
            self.game_state = bitcode::deserialize(
                &std::fs::read("state.bin").expect("failed to read state file")
            ).expect("failed to deserialize state file");
        }
    }

    
}