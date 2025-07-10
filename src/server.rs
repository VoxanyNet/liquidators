use std::{net::SocketAddr, time::Duration};

use gamelibrary::sync::server::SyncServer;
use crate::{game_state::GameState, level::Level};

pub struct Server {
    pub sync_server: SyncServer<GameState>,
    pub previous_tick_player_count: usize
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {

        let mut game_state = GameState::empty();

        game_state.level = Level::from_save("level.yaml".to_string());

        let sync_server = SyncServer::new(address, game_state);
        
        Self {
            sync_server,
            previous_tick_player_count: 0
        }

        
    }

    pub fn reset_level_if_no_players(&mut self) {
        if self.sync_server.client_count() == 0 && self.previous_tick_player_count != 0 {

            println!("resetting level!");

            self.sync_server.state_mut().level = Level::from_save("level.yaml".to_string());
        }
    }

    pub fn run(&mut self) {

        loop {

            self.previous_tick_player_count = self.sync_server.client_count();

            self.sync_server.accept_new_client();

            self.sync_server.receive_updates();

            self.reset_level_if_no_players();
            
            

            // slow the loop down a bit so that it doesnt use so much cpu
            std::thread::sleep(web_time::Duration::from_micros(200));
            
        }
    }


}

