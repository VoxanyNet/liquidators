use std::{fs, time::Instant};

use gamelibrary::{log, menu::Button, sync::client::SyncClient, texture_loader::TextureLoader, uuid};
use liquidators_lib::level::Level;
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{DARKGRAY, WHITE}, input::{self, is_key_released, is_mouse_button_down, mouse_delta_position, mouse_wheel}, math::Rect, text::draw_text, time::get_fps, window::{screen_height, screen_width}};
use gamelibrary::traits::HasPhysics;

pub struct EditorClient {
    pub uuid: String,
    pub level: Level,
    pub save_button: Button,
    pub load_button: Button,
    pub camera_rect: Rect,
    pub sync_client: SyncClient<Level>,
    pub textures: TextureLoader,
    pub last_tick: Instant,
    pub enable_physics: bool
}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 

impl EditorClient {

    pub async fn connect(url: &str) -> Self {

        let uuid = uuid();

        let (sync_client, level): (SyncClient<Level>, Level) = SyncClient::connect(url).await;
        
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

        let camera_rect = Rect::new(0., 200., screen_width() / 1.50, screen_height() / 1.5);

        Self {
            uuid,
            level,
            save_button,
            load_button,
            camera_rect,
            sync_client,
            textures: TextureLoader::new(),
            last_tick: Instant::now(),
            enable_physics: false

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

            for shotgun in &self.level.shotguns {
                owned_colliders.push(shotgun.collider);
                owned_rigid_bodies.push(shotgun.rigid_body)
            }

            for structure in &self.level.structures {
                owned_rigid_bodies.push(structure.rigid_body_handle);
                owned_colliders.push(structure.collider_handle);
            }

            for brick in &self.level.bricks {
                owned_rigid_bodies.push(brick.rigid_body_handle().clone());
                owned_colliders.push(brick.collider_handle().clone());
            }

            self.level.space.step(&owned_rigid_bodies, &owned_colliders, &self.last_tick);
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

        self.level.editor_tick(&self.camera_rect, &self.uuid);

        self.toggle_physics();

        self.update_camera();

        self.handle_buttons();
        
        self.handle_menus();

        self.step_space();   

        self.last_tick = Instant::now();         

    }

    pub fn handle_menus(&mut self) {
        // this needs to be a function on the editor struct because structures cannot delete themselves

        let mut structure_index = 0;
        let mut structures_length = self.level.structures.len();

        loop {

            if structure_index >= structures_length {
                break;
            }

            if self.level.structures[structure_index].editor_owner != self.uuid {
                structure_index += 1;
                continue;
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

        self.level.editor_draw(&mut self.textures).await;

        set_default_camera();

        draw_text(format!("fps: {}", get_fps()).as_str(), screen_width() - 120., 25., 30., WHITE);

        self.save_button.draw().await;
        self.load_button.draw().await;

        

    }

    pub async fn run(&mut self) {

        // let mut sound = Sound::new("assets/sounds/radio.mp3").unwrap();
        // sound.set_volume(0.25);

        // sound.play();

        //macroquad::window::set_fullscreen(true);

        loop { 

            log(format!("{}", self.level.structures.len()).as_str());

            self.tick();

            self.draw().await;

            self.sync_client.sync(&mut self.level);

            macroquad::window::next_frame().await;
        }

    }
}