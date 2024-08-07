use std::{net::SocketAddr, time::Duration};

use gamelibrary::sync::server::SyncServer;
use liquidators_lib::level::Level;

pub struct EditorServer {
    pub sync_server: SyncServer<Level>
}

impl EditorServer {
    pub fn new(address: SocketAddr) -> Self {
        let level = Level::empty();

        let sync_server = SyncServer::new(address, level);

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