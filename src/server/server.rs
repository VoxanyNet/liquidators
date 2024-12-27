use std::{net::SocketAddr, time::Duration};

use gamelibrary::sync::server::SyncServer;
use liquidators_lib::{game_state::GameState, level::Level};

pub struct Server {
    pub sync_server: SyncServer<GameState>
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {

        let mut game_state = GameState::empty();

        game_state.level = Level::from_save("level.yaml".to_string());

        let sync_server = SyncServer::new(address, game_state);
        
        Self {
            sync_server
        }

        
    }

    pub fn run(&mut self) {

        loop {
            
            self.sync_server.accept_new_client();

            self.sync_server.receive_updates();

            // slow the loop down a bit so that it doesnt use so much cpu
            std::thread::sleep(Duration::from_micros(1));
            
        }
    }


}

