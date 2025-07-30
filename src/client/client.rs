use std::{fs, net::SocketAddr, str::FromStr, sync::{mpsc, Arc, Mutex}, thread::{sleep, Thread}, time::{Duration, Instant}};

use futures::{executor::block_on, future::Select};
use gamelibrary::{animation_loader::AnimationLoader, arenaiter::SyncArenaIterator, font_loader::FontLoader, log, mouse_world_pos, rapier_mouse_world_pos, sound::soundmanager::SoundManager, sync::client::SyncClient, texture_loader::TextureLoader, time::Time, traits::HasPhysics, uuid_string};
use gilrs::GamepadId;
use liquidators_lib::{console::Console, editor_client::EditorClient, editor_server::EditorServer, game_state::GameState, level::Level, main_menu::MainMenu, player::player::Player, server::Server, vec_remove_iter::IntoVecRemoveIter, ScreenShakeParameters, TickContext};
use macroquad::{audio::set_sound_volume, camera::{set_camera, set_default_camera, Camera2D}, color::WHITE, input::{self, is_key_down, is_key_released, is_mouse_button_down, is_quit_requested, mouse_delta_position, mouse_position, mouse_wheel, prevent_quit, KeyCode}, math::{vec2, Rect, Vec2}, prelude::{camera::mouse, gl_use_default_material, gl_use_material, load_material, MaterialParams, PipelineParams, ShaderSource, UniformDesc, UniformType}, text::{draw_text, draw_text_ex, TextParams}, texture::{draw_texture_ex, DrawTextureParams}, time::get_fps, window::{next_frame, request_new_screen_size, screen_height, screen_width}};
use noise::{NoiseFn, Perlin};
use tungstenite::http::request;

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(feature = "3d-audio")]
use gamelibrary::sound::backends::ears::EarsSoundManager as SelectedSoundManager; // this alias needs a better name

#[cfg(not(feature = "3d-audio"))]
use gamelibrary::sound::backends::macroquad::MacroquadSoundManager as SelectedSoundManager;

pub struct Client {
    pub game_state: GameState,
    pub is_host: bool,
    pub textures: TextureLoader,
    pub animations: AnimationLoader,
    pub last_tick: web_time::Instant,
    pub uuid: String,
    pub camera_offset: Vec2,
    pub update_count: i32,
    pub sync_client: Option<SyncClient<GameState>>,
    pub last_sync: web_time::Instant,
    pub camera_rect: Rect,
    pub active_gamepad: Option<GamepadId>,
    pub console: Console,
    pub sounds: SelectedSoundManager,
    pub last_tick_mouse_world_pos: Vec2,
    pub main_menu: Option<MainMenu>,
    pub font_loader: FontLoader,
    pub start: web_time::Instant,
    pub screen_shake: ScreenShakeParameters,
    pub last_tick_duration: Duration
}

impl Client {
    
    pub async fn sync_sounds(&mut self, ctx: &mut TickContext<'_>) {
        self.game_state.sync_sounds(ctx).await
    }

    pub async fn tick(&mut self) {

        // set ourselves as host if we are the only player connected
        if self.game_state.level.players.len() == 1 {
            self.is_host = true;
        }

        self.resize_camera();

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
            camera_rect: &mut self.camera_rect,
            active_gamepad: &self.active_gamepad,
            console: &mut self.console,
            owned_rigid_bodies: &mut vec![],
            owned_colliders: &mut vec![],
            owned_impulse_joints: &mut vec![],
            sounds: &mut self.sounds,
            last_tick_mouse_world_pos: &mut self.last_tick_mouse_world_pos,
            font_loader: &mut self.font_loader,
            screen_shake: &mut self.screen_shake,
            last_tick_duration: self.last_tick_duration
        };

        self.game_state.tick(
            &mut tick_context
        );

        self.game_state.sync_sounds(&mut tick_context).await;

        if let Some(menu) = &mut self.main_menu {
            menu.tick(&mut tick_context);

            if menu.new_game {

                let i = 0;
                
                let mut server = Server::new(SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5556));

                let server_thread = std::thread::spawn(move ||
                    server.run()
                );


                // temporary workarond so that the menu click doesnt count in game
                //std::thread::sleep(web_time::Duration::from_secs_f32(0.2));
                next_frame().await;

                let mut client = Client::connect("ws://127.0.0.1:5556").await;

                std::mem::swap(&mut client.textures, &mut self.textures);
                std::mem::swap(&mut client.sounds, &mut self.sounds);
                std::mem::swap(&mut client.font_loader, &mut self.font_loader);

                *self = client;


            }

            else if menu.connect {
                // temporary workarond so that the menu click doesnt count in game
                //std::thread::sleep(web_time::Duration::from_secs_f32(0.2));
                next_frame().await;

                // connect locally if running natively
                let ip = "ws://127.0.0.1:6969";

                #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
                let ip = "wss://liquidators.voxany.net/ws/";

                let mut client = Client::connect(ip).await;

                // sneaky sneaky. need to transfer the existing preloaded assests to the new client but borrow checker doesnt like that
                std::mem::swap(&mut client.textures, &mut self.textures);
                std::mem::swap(&mut client.sounds, &mut self.sounds);
                std::mem::swap(&mut client.font_loader, &mut self.font_loader);

                *self = client;
            }

            // else if menu.launch_editor {
            //     let mut editor_server = EditorServer::new(SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5557));
                
            //     let editor_server_thread = std::thread::spawn(move ||
            //         editor_server.run()
            //     );

            //     let mut editor_client = EditorClient::connect("ws://127.0.0.1:5557").await;

            //     editor_client.run().await;

            //     editor_server_thread.join().unwrap();
            
            // }


        }

        // if is_key_released(KeyCode::R) {
        //     self.reset_level();
        // }
        
        self.update_camera();

        // if is_key_released(KeyCode::B) {
        //     self.game_state.level = Level::from_save("level.yaml".to_string());
        // }

        self.save_state();

        self.last_tick_duration = self.last_tick.elapsed();
        self.last_tick = web_time::Instant::now();

        // THIS IS MEGA STUPID like actually so dumb
        if self.start.elapsed() > Duration::from_secs_f32(1.) {
            self.sounds.set_stupid_connection_fix(false);
        }
        

    }

    pub fn resize_camera(&mut self) {
        //self.game_state.chat.add_message("debug".to_string(), format!("{}, {}", screen_width(), screen_height()));

        
        self.camera_rect.w = screen_width();
        self.camera_rect.h = screen_height();
    }

    pub fn update_camera(&mut self) {

        let current_mouse_pos = rapier_mouse_world_pos(&self.camera_rect);

        let mouse_delta = Vec2::new(
            self.last_tick_mouse_world_pos.x - current_mouse_pos.x, 
            self.last_tick_mouse_world_pos.y - current_mouse_pos.y
        );

        if mouse_wheel().1 < 0. {
            self.camera_rect.w *= 1.1;
            self.camera_rect.h *= 1.1;
        }

        if mouse_wheel().1 > 0. {

            self.camera_rect.w /= 1.1;
            self.camera_rect.h /= 1.1;
        }

        if is_key_down(KeyCode::LeftControl) {
            self.camera_rect.x += mouse_delta.x;
            self.camera_rect.y += mouse_delta.y;
        }
    }

    pub fn disconnect(&mut self) {

        
       let mut players_iter = SyncArenaIterator::new(&mut self.game_state.level.players);

       while let Some((player, _)) = players_iter.next() {

            if player.owner != self.uuid {
                players_iter.restore(player);
            }

            else {
                player.despawn(&mut self.game_state.level.space);
            }
        }


        // send a final sync to the server
        self.sync_client.as_mut().unwrap().sync(&mut self.game_state);

        self.sync_client.as_mut().unwrap().disconnect();

    }

    pub fn reset_level(&mut self) {
        log("resetting");
        let reset_level: Level = serde_yaml::from_str(&fs::read_to_string("level.yaml").unwrap()).unwrap();

        self.game_state.level = reset_level;

        for (_, structure) in self.game_state.level.structures.iter_mut() {
            structure.owner = Some(self.uuid.clone())
        }

        for brick in self.game_state.level.bricks.iter_mut() {
            brick.owner = Some(self.uuid.clone());
        }

        Player::spawn(&mut self.game_state.level.players, &mut self.game_state.level.space, self.uuid.clone(), &vec2(100., 300.), &mut self.textures);

    }

    pub async fn run(&mut self) {

        prevent_quit();

        // let shader = load_material(
        //     ShaderSource::Glsl {
        //         vertex: include_str!("default_share.glsl"),
        //         fragment: include_str!("shader.glsl"),
        //     },
        //     MaterialParams {
        //         uniforms: vec![
        //             UniformDesc { name: "screen_size".to_string(), uniform_type: UniformType::Float2, array_count: 1 },
        //         ],
        //         textures: vec!["texture".to_string()],
        //         pipeline_params: PipelineParams::default()
        //     },
        // )   
        // .unwrap();
        
        loop {

            

            if is_quit_requested() {
                self.disconnect();



                

                break;
            }

            
            //let then =web_time::Instant::now();

            // only tick maximum 120 times per second to avoid glitchyness
            if self.last_tick.elapsed().as_millis() >= 8 {

                //println!("{}", self.last_tick.elapsed().as_millis() - 8);

                let then = web_time::Instant::now();
                self.tick().await;

                //println!("tick: {:?}", then.elapsed());
            

                
                self.draw().await;

                
                
                // self.tick() updates self.last_tick automatically unlike self.last_sync
            }
            
            // only sync 30 tps
            // this could probably be optimized but this is more readable

            if !is_key_down(KeyCode::M) {
                if self.last_sync.elapsed().as_secs_f32() > 1./120. {

                    if let Some(sync_client) = &mut self.sync_client {

                        let then = web_time::Instant::now();
                        
                        sync_client.sync(&mut self.game_state);

                        //println!("sync: {:?}", then.elapsed());
                    }

                    self.last_sync =web_time::Instant::now();

                }   

            }
            
            //println!("FPS: {}", 1. / then.elapsed().as_secs_f64());
            
            if is_key_released(KeyCode::H) {
                log("paused");
            }

            self.last_tick_mouse_world_pos = rapier_mouse_world_pos(&self.camera_rect);         

            // calculate the time we need to sleep to lock the framerate at 120
            //let sleep_duration = Duration::from_millis(7).saturating_sub(self.last_tick.elapsed());
            

            //std::thread::sleep(sleep_duration);
    
        }
    }

 
    pub async fn draw(&mut self) {

        
        let then =web_time::Instant::now();

        
        //draw_text(format!("uuid: {}", self.uuid).as_str(), screen_width() - 120., 25., 30., WHITE);
        
        let elapsed = self.start.elapsed().as_secs_f64();

        let x_shake = {
            let frequency_modifier = self.screen_shake.x_frequency;
            
            let magnitude_modifier = self.screen_shake.x_intensity;

            let offset = self.screen_shake.x_offset;

            magnitude_modifier * ((frequency_modifier * elapsed) + offset).sin()

            
        };

        let y_shake = {
            let frequency_modifier = self.screen_shake.y_frequency;
            
            let magnitude_modifier = self.screen_shake.y_intensity;

            let offset = self.screen_shake.y_offset;

            magnitude_modifier * ((frequency_modifier * elapsed) + offset).sin()
        };
        
        // add shake
        let shaken_camera_rect = Rect {
            x: self.camera_rect.x + x_shake as f32,
            y: self.camera_rect.y + y_shake as f32,
            w: self.camera_rect.w,
            h: self.camera_rect.h,
        };


        let mut camera = Camera2D::from_display_rect(shaken_camera_rect);

        camera.zoom.y = -camera.zoom.y;

        set_camera(
            &camera
        );


        // apply decays
        let x_frequency_decay = self.screen_shake.x_frequency_decay * self.last_tick_duration.as_secs_f64();
        let y_frequency_decay = self.screen_shake.y_frequency_decay * self.last_tick_duration.as_secs_f64();

        let x_intensity_decay = self.screen_shake.x_intensity_decay * self.last_tick_duration.as_secs_f64();
        let y_intensity_decay = self.screen_shake.y_intensity_decay * self.last_tick_duration.as_secs_f64();

        self.screen_shake.x_frequency = (self.screen_shake.x_frequency - x_frequency_decay).max(0.0);
        self.screen_shake.y_frequency = (self.screen_shake.y_frequency - y_frequency_decay).max(0.0);

        self.screen_shake.x_intensity = (self.screen_shake.x_intensity - x_intensity_decay).max(0.0);
        self.screen_shake.y_intensity = (self.screen_shake.y_intensity - y_intensity_decay).max(0.0);

    
        self.game_state.draw(&mut self.textures, &self.camera_rect, &mut self.font_loader, &camera).await;

        set_default_camera();

        let mut tick_context = TickContext {
            is_host: &mut self.is_host,
            textures: &mut self.textures,
            uuid: &self.uuid,
            camera_offset: &mut self.camera_offset,
            last_tick: &self.last_tick,
            camera_rect: &mut self.camera_rect,
            active_gamepad: &self.active_gamepad,
            console: &mut self.console,
            owned_rigid_bodies: &mut vec![],
            owned_colliders: &mut vec![],
            owned_impulse_joints: &mut vec![],
            sounds: &mut self.sounds,
            last_tick_mouse_world_pos: &mut self.last_tick_mouse_world_pos,
            font_loader: &mut self.font_loader,
            screen_shake: &mut self.screen_shake,
            last_tick_duration: self.last_tick_duration
        };

        self.game_state.draw_hud(&mut tick_context).await;

        self.console.draw().await;

        self.game_state.chat.draw().await;

        if let Some(main_menu) = &self.main_menu {
            main_menu.draw(&mut self.textures, &mut self.font_loader).await
        }

        //println!("draw: {:?}", then.elapsed());
        
        draw_text(format!("fps: {}", get_fps()).as_str(), screen_width() - 120., 25., 30., WHITE);

        if is_key_down(KeyCode::LeftAlt) {
            let macroquad_screen_mouse_pos = mouse_position();
            let macroquad_world_mouse_pos = mouse_world_pos(&self.camera_rect);
            let rapier_world_mouse_pos = rapier_mouse_world_pos(&self.camera_rect);

            draw_text(format!("screen: {}, {}", macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1), macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1 + 15., 20., WHITE);
            draw_text(format!("macroquad world: {}, {}", macroquad_world_mouse_pos.x, macroquad_world_mouse_pos.y ), macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1 + 30., 20., WHITE);
            draw_text(format!("rapier world: {}, {}", rapier_world_mouse_pos.x, rapier_world_mouse_pos.y), macroquad_screen_mouse_pos.0, macroquad_screen_mouse_pos.1 + 45., 20., WHITE);
        }
        
        let epic = self.textures.get(&"assets/structure/brick_block.png".to_string()).await;

        // let tile_size = 64.0; // Size of the texture tile
        // let rect_width = 256.0;
        // let rect_height = 256.0;

        // let tiles_x = rect_width / tile_size;
        // let tiles_y = rect_height / tile_size;

        
        // draw_texture_ex(
        //     epic,
        //     100.0, // x
        //     100.0, // y
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(rect_width, rect_height)),
        //         source: Some(Rect::new(
        //             0.0,
        //             0.0,
        //             (epic.width() * tiles_x) - 20.,
        //             (epic.height() * tiles_y) - 20.,
        //         )),
        //         ..Default::default()
        //     },
        // );

        macroquad::window::next_frame().await;
    }

    pub async fn new_unconnected() -> Self {
        
        let mut textures = TextureLoader::new();
        let mut sound_manager = SelectedSoundManager::new();
        let mut font_loader = FontLoader::new();

        
        let font = font_loader.get("assets/fonts/CutePixel.ttf").await.clone();
        let mut text_params = TextParams::default();
        text_params.font = Some(&font);
        text_params.font_size = 40;

        // preload assets
        for (index, asset_path) in ASSET_PATHS.iter().enumerate() {

            let text = format!("Loading: %{}", ((index as f32 / ASSET_PATHS.len() as f32) * 100.) as u32);

            draw_text_ex(&text, (screen_width() / 2.) - 110., screen_height() / 2., text_params.clone());

            if asset_path.ends_with(".png") {
                textures.get(&asset_path.to_string()).await;
            }
            
            if asset_path.ends_with(".wav") {
                sound_manager.load_sound(asset_path).await;
            }

            if asset_path.ends_with(".ttf") {
                font_loader.load(&asset_path).await;

            }

            next_frame().await
            
        }

    
        

        let main_menu = MainMenu::new(&mut textures).await;
        Self {
            game_state: GameState::empty(),
            is_host: true,
            textures: textures,
            animations: AnimationLoader::new(),
            last_tick:web_time::Instant::now(),
            uuid: uuid_string(),
            camera_offset: Vec2::ZERO,
            update_count: 0,
            sync_client: None,
            last_sync:web_time::Instant::now(),
            camera_rect: Rect::new(0., 200., 1280., 720.),
            active_gamepad: None,
            console: Console::new(),
            sounds: sound_manager,
            last_tick_mouse_world_pos: rapier_mouse_world_pos(&Rect::new(0., 200., 1280., 720.)),
            main_menu: Some(main_menu),
            font_loader: font_loader,
            start:web_time::Instant::now(),
            screen_shake: ScreenShakeParameters::default(None, None),
            last_tick_duration: web_time::Duration::new(0, 500)

        }
    }
    pub async fn connect(url: &str) -> Self {


        let mut textures = TextureLoader::new();

        let camera_rect = Rect::new(0., 200., 1280., 720.);
        
        let uuid = uuid_string();



        
        let (sync_client, mut game_state): (SyncClient<GameState>, GameState) = SyncClient::connect(url).await;

        // if we are the first player to join, we take ownership of everything
        if game_state.level.players.len() == 0 {
            for (_, structure) in game_state.level.structures.iter_mut() {
                structure.owner = Some(uuid.clone())
            }

            for brick in game_state.level.bricks.iter_mut() {
                brick.owner = Some(uuid.clone());
            }
            
            for shotgun in game_state.level.shotguns.iter_mut() {
                shotgun.set_owner(uuid.clone());
            }
        }

        // if we are the only player when connecting we are the host
        let is_host = game_state.level.players.len() == 0;

        Player::spawn(&mut game_state.level.players, &mut game_state.level.space, uuid.clone(), &vec2(100., 300.), &mut textures);

        //let gilrs = Gilrs::new().unwrap();

        let active_gamepad: Option<GamepadId> = None; 

        // active_gamepad = gilrs.gamepads().next().map_or(None, |gamepad|{Some(gamepad.0)});

        let mut sounds = SelectedSoundManager::new();

        sounds.set_stupid_connection_fix(true);
        
        Self {
            game_state,
            is_host,
            textures, 
            animations: AnimationLoader::new(),
            last_tick:web_time::Instant::now(),
            uuid,
            camera_offset: Vec2::new(0., 0.),
            update_count: 0,
            last_sync:web_time::Instant::now(),
            camera_rect,
            active_gamepad,
            sync_client: Some(sync_client),
            console: Console::new(),
            sounds: sounds,
            last_tick_mouse_world_pos: rapier_mouse_world_pos(&camera_rect),
            main_menu: None,
            font_loader: FontLoader::new(),
            start:web_time::Instant::now(),
            screen_shake: ScreenShakeParameters::default(None, None),
            last_tick_duration: web_time::Duration::new(0, 500)
        }
    }

    fn save_state(&mut self) {
        if is_key_released(macroquad::input::KeyCode::F5) {

            let game_state_json = serde_yaml::to_string(&self.game_state).unwrap();

            std::fs::write("state.yaml", game_state_json).unwrap();
        }

        if is_key_released(macroquad::input::KeyCode::F6) {
            self.game_state = serde_yaml::from_str(
                &std::fs::read_to_string("state.yaml").expect("failed to read state file")
            ).expect("failed to deserialize state file");
        }
    }

    
}