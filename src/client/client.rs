use std::{collections::HashMap, fs, io::Read, time::{SystemTime, UNIX_EPOCH}};

use gamelibrary::{macroquad_to_rapier, space::SpaceDiff, time::Time};
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
    pub last_tick_game_state: GameState,
    pub textures: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, macroquad::audio::Sound>,
    pub last_tick: Time,
    pub uuid: String,
    pub server_receive: ewebsock::WsReceiver,
    pub server_send: ewebsock::WsSender,
    pub camera_offset: Vec2,
    pub update_count: i32,
    pub start_time: Time,
    pub square_color: Color
}

impl Client {

    pub async fn run(&mut self) {

        loop {
        
            //macroquad::window::clear_background(macroquad::color::BLACK);


            if is_key_released(KeyCode::Q) {

                let game_state_diff_bytes = fs::read("diff.bin").unwrap();
                let game_state_diff_bytes = game_state_diff_bytes.as_slice();
                let game_state_diff: SpaceDiff = bitcode::deserialize(game_state_diff_bytes).unwrap();
                self.game_state.space.apply(
                    &game_state_diff
                )
            }

            let rapier_mouse_coords = macroquad_to_rapier( 
                &Vec2::new(mouse_position().0, mouse_position().1)
            );

            self.game_state.space.query_pipeline.intersections_with_shape(&self.game_state.space.rigid_body_set,
                &self.game_state.space.collider_set, &vector![rapier_mouse_coords.x, rapier_mouse_coords.y].into(), ColliderBuilder::cuboid(10., 10.).build().shape(), QueryFilter::default(), |handle| {
                    println!("The collider {:?} intersects our shape.", handle);
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    println!("{:?}", since_the_epoch);
                    true // Return `false` instead if we want to stop searching for other colliders that contain this point.
                }
            );
    
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

            if is_key_released(KeyCode::K) {
                let physics_square = PhysicsSquare::new(
                    &mut self.game_state.space,
                    Vec2::new(50., 500.),
                    RigidBodyType::Dynamic,
                    20., 
                    20., 
                    &self.uuid.clone(),
                    true,
                    self.square_color
                );
            
                self.game_state.physics_squares.push(physics_square);
            }
            
            
            self.draw().await;

            self.send_updates();
            
            self.receive_updates();

            // we dont want to track the changes that happen to the game state when we receive updates
            // so we set the checkpoint right after we receive the updates
            // this way it will only track what happened when we ticked the game state
            self.last_tick_game_state = self.game_state.clone();
    
            macroquad::window::next_frame().await;
    
            if macroquad::input::is_key_down(macroquad::input::KeyCode::J) {
                let state_string = serde_json::to_string_pretty(&self.game_state).unwrap();
    
                std::fs::write("state.json", state_string).expect("failed to write current state to state.json")
            }
    
            // cap framerate at 200fps (or 5 ms per frame)
            // TODO: this needs to take into account the time it took to draw the last frame
            // std::thread::sleep(
            //     Duration::from_millis(5)
            // );



        }
    }

    // generate and send diff of game state from the last time we called this function (previous tick)
    pub fn send_updates(&mut self) {

        if self.last_tick_game_state == self.game_state {
            //println!("no changes in game state, not sending update");
            return;
        }
        let diff = self.last_tick_game_state.diff(&self.game_state);

        // let diff_string = match serde_json::to_string(&diff) {
        //     Ok(diff_string) => diff_string,
        //     Err(error) => panic!("failed to serialize game state diff: {}", error),
        // };

        let diff_bytes = bitcode::serialize(&diff).expect("failed to serialize game state diff");

        let compressed_diff_bytes = compress_prepend_size(&diff_bytes);

        self.server_send.send(ewebsock::WsMessage::Binary(compressed_diff_bytes.to_vec()));

    }

    pub fn receive_updates(&mut self) {
        // let mut update_count = 0;
        
        // we loop until there are no new updates
        loop {

            let compressed_game_state_diff_bytes = match self.server_receive.try_recv() {
                Some(event) => {
                    match event {
                        ewebsock::WsEvent::Opened => todo!("unhandled 'Opened' event"),
                        ewebsock::WsEvent::Message(message) => {
                            match message {
                                ewebsock::WsMessage::Binary(bytes) => bytes,
                                _ => todo!("unhandled message type when trying to receive updates from server")
                            }
                        },
                        ewebsock::WsEvent::Error(error) => todo!("unhandled 'Error' event when trying to receive update from server: {}", error),
                        ewebsock::WsEvent::Closed => todo!("server closed"),
                    }
                },
                None => break, // this means there are no more updates
            };
            
            let game_state_diff_bytes = decompress_size_prepended(&compressed_game_state_diff_bytes).expect("Failed to decompress incoming update");

            let game_state_diff: GameStateDiff = match bitcode::deserialize(&game_state_diff_bytes) {
                Ok(game_state_diff) => game_state_diff,
                Err(error) => {
                    panic!("failed to deserialize game state diff: {}", error);
                },
            };

            self.game_state.apply(&game_state_diff);

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

        let (server_send, server_receive) = match ewebsock::connect(url, ewebsock::Options::default()) {
            Ok(result) => result,
            Err(error) => {
                panic!("failed to connect to server: {}", error)
            },
        }; 

        // wait for Opened event from server
        loop {
            match server_receive.try_recv() {
                Some(event) => {
                    match event {
                        ewebsock::WsEvent::Opened => {
                            println!("we got the opened message!");
                            break;
                        },
                        ewebsock::WsEvent::Message(message) => {
                            match message {
                                _ => panic!("received a message from the server")
                            }
                        },
                        ewebsock::WsEvent::Error(error) => panic!("received error when trying to connect to server: {}", error),
                        ewebsock::WsEvent::Closed => panic!("server closed when trying to connect"),
                        
                    }
                },
                None => continue,
            }
        }

        let compressed_game_state_string_bytes = loop {

            match server_receive.try_recv() {
                Some(event) => {
                    match event {
                        ewebsock::WsEvent::Opened => todo!("unhandled opened event on connect"),
                        ewebsock::WsEvent::Message(message) => {
                            match message {
                                ewebsock::WsMessage::Binary(bytes) => break bytes,
                                _ => todo!("unhandled message type when receiving initial state")
                            }
                        },
                        ewebsock::WsEvent::Error(error) => todo!("unhandled error when receiving initial state: {}", error),
                        ewebsock::WsEvent::Closed => todo!("unhandled closed event when receiving initial state"),
                    }
                },
                None => continue, // this means that the server would have blocked, so we try again
            };
        };
        
        let game_state_bytes = decompress_size_prepended(&compressed_game_state_string_bytes).expect("Failed to decompress initial state");

        let game_state: GameState = match bitcode::deserialize(&game_state_bytes) {
            Ok(game_state_diff) => game_state_diff,
            Err(error) => {
                panic!("failed to deserialize game state diff: {}", error);
            },
        };
        
        Self {
            game_state: game_state.clone(),
            is_host: true,
            last_tick_game_state: game_state.clone(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
            last_tick: Time::now(),
            uuid: uuid,
            server_receive: server_receive,
            server_send: server_send,
            camera_offset: Vec2::new(0., 0.),
            update_count: 0,
            start_time: Time::now(),
            square_color: random_color()
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

    pub fn tick(&mut self) {

        self.control_camera();
        if is_key_released(macroquad::input::KeyCode::F5) {

            let game_state_binary = bitcode::serialize(&self.game_state).unwrap();
            let game_state_yaml = serde_yaml::to_string(&self.game_state).unwrap();

            std::fs::write("state.bin", game_state_binary).unwrap();
            std::fs::write("state.yaml", game_state_yaml).unwrap();
        }

        if is_key_released(macroquad::input::KeyCode::F6) {
            self.game_state = bitcode::deserialize(
                &std::fs::read("state.bin").expect("failed to read state file")
            ).expect("failed to deserialize state file");
        }

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

  
        for index in 0..self.game_state.physics_squares.len() {


            // take the player out, tick it, then put it back in
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