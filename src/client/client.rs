use std::{collections::HashMap, net::TcpStream, time::Duration};

use game::{entities::Entity, game::{Drawable, HasOwner, Texture, TickContext, Tickable}, game_state::GameState, networking::receive_headered, proxies::uuid::lib::Uuid, time::Time};
use macroquad::texture::Texture2D;

pub struct Client {
    pub game_state: GameState,
    pub is_host: bool,
    pub last_tick_game_state: GameState,
    pub textures: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, macroquad::audio::Sound>,
    pub last_tick: Time,
    pub uuid: Uuid,
    pub server: TcpStream
}

impl Client {

    pub async fn run(&mut self) {
        loop {
        
            //macroquad::window::clear_background(macroquad::color::BLACK);
    
            self.tick();
            
            self.draw().await;
    
            macroquad::window::next_frame().await;
    
            if macroquad::input::is_key_down(macroquad::input::KeyCode::J) {
                let state_string = serde_json::to_string_pretty(&self.game_state).unwrap();
    
                std::fs::write("state.json", state_string).expect("failed to write current state to state.json")
            }
    
            // cap framerate at 200fps (or 5 ms per frame)
            // TODO: this needs to take into account the time it took to draw the last frame
            std::thread::sleep(
                Duration::from_millis(5)
            );
    
            //println!("{}",  macroquad::time::get_fps());
        }
    }
    pub async fn draw(&mut self) {
        for entity in self.game_state.entities.iter_mut() {

            match entity {
                Entity::Player(player) => {player.draw(&mut self.textures).await}
                Entity::Zombie(zombie) => {zombie.draw(&mut self.textures).await}
                Entity::Bullet(bullet) => {bullet.draw()},
                Entity::Coin(coin) => {coin.draw()},
                Entity::Tree(tree) => {tree.draw(&mut self.textures).await},
                Entity::Wood(wood) => {wood.draw(&mut self.textures).await}
            };
        }
    }

    pub fn connect(address: &str) -> Self {

        let uuid: Uuid = uuid::Uuid::new_v4().into();

        let mut server = match TcpStream::connect(address) {
            Ok(stream) => stream,
            Err(error) => {
                panic!("failed to connect: {}", error)
            },
        };

        let game_state_string_bytes = match receive_headered(&mut server) {
            Ok(game_state_diff_string_bytes) => game_state_diff_string_bytes,
            Err(error) => {
                panic!("failed to receive game state update from server: {}", error);
            },
        };
        
        let game_state_string = match String::from_utf8(game_state_string_bytes.clone()) {
            Ok(game_state_diff_string) => game_state_diff_string,
            Err(error) => {
                panic!("failed to decode game state diff as string {}", error);
            },
        };

        let game_state: GameState = match serde_json::from_str(&game_state_string) {
            Ok(game_state_diff) => game_state_diff,
            Err(error) => {
                panic!("failed to deserialize game state diff: {}", error);
            },
        };

        server.set_nonblocking(true).expect("failed to set socket as non blocking");
        
        Self {
            game_state: game_state.clone(),
            is_host: true,
            last_tick_game_state: game_state.clone(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
            last_tick: Time::now(),
            uuid: uuid,
            server: server
        }
    }


    pub fn connect_as_master() {

    }

    // pub fn connect(ip: &str) -> Self {
    //     let server = match TcpStream::connect(ip) {
    //         Ok(server) => server,
    //         Err(error) => panic!("failed to connect to server address {}: {}", ip, error),
    //     };

    //     Self {}
    // }

    pub fn tick(&mut self) {

        // we create a tick context because we cannot pass Client directly
        // we want others to be able to create their own client structs so TickContext is the middle man
        let mut tick_context = TickContext {
            game_state: &mut self.game_state,
            is_host: &mut self.is_host,
            textures: &mut self.textures,
            sounds: &mut self.sounds,
            last_tick: &mut self.last_tick,
            uuid: &mut self.uuid,
        };

        for index in 0..tick_context.game_state.entities.len() {

            // take the player out, tick it, then put it back in
            let mut entity = tick_context.game_state.entities.swap_remove(index);

            // we only tick the entity if we own it
            if entity.get_owner() == *tick_context.uuid {
                entity.tick(&mut tick_context);
            }
            
            // put the entity back
            tick_context.game_state.entities.push(entity);

        }

        self.last_tick = Time::now(); 

    }
}