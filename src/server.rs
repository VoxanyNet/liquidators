use std::{net::{SocketAddr, TcpListener, TcpStream}, time::Duration};

use gamelibrary::sync::server::SyncServer;
use lz4_flex::decompress_size_prepended;
use tungstenite::{Message, WebSocket};
use crate::{game_state::GameState, level::Level, updates::Update};

pub struct Server {
    game_state: GameState,
    player_count: u32,
    listener: TcpListener,
    clients: Vec<WebSocket<TcpStream>>,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {

        let mut game_state = GameState::empty();

        game_state.level = Level::from_save("level.yaml".to_string());

        let listener = match TcpListener::bind(address) {
            Ok(listener) => listener,
            Err(error) => panic!("failed to bind listener: {}", error),
        };

        match listener.set_nonblocking(true) {
            Ok(_) => {},
            Err(error) => panic!("failed to set server as non blocking: {}", error),
        };
        
        Self {
            game_state,
            player_count: 0,
            listener,
            clients: Vec::new(),
            
        }

        
    }

    pub fn receive_updates(&mut self) {

        let mut client_index = 0;

        'client_loop: while client_index < self.clients.len() {

            // take the client out, receive all updates, then put it back in
            let mut client = self.clients.remove(client_index);
            
            // keep trying to receive updates until there are none
            loop {

                let compressed_update_bytes = match client.read() {
                    Ok(message) => {
                        match message {
                            Message::Binary(compressed_update_bytes) => {
                                compressed_update_bytes
                            },
                            Message::Close(_close_message) => {

                                println!("client {} disconnected", client_index);
                                continue 'client_loop;
                            },
                            _ => {
                                println!("client tried to send non binary message. disconnecting them!");

                                continue 'client_loop;
                            }
                        }
                    },
                    Err(error) => {
                        match error {

                            tungstenite::Error::Io(io_error) => {
                                match io_error.kind() {
                                    std::io::ErrorKind::WouldBlock => {
                                        // this means that there was no update to read
                                        self.clients.insert(client_index, client);
                                        
                                        client_index += 1;
                                        
                                        continue 'client_loop // move to the next client
                                    },
                                    std::io::ErrorKind::ConnectionReset => {
                                        println!("client {} disconnected", client_index);

                                        // do not increment client index because we arent putting this one back

                                        continue 'client_loop;
                                    }
                                    _ => todo!("unhandled io error: {}", io_error),
                                }
                            },
                            
                            tungstenite::Error::Protocol(_error) => {
                                println!("client {} disconnected due to protocol error", client_index);

                                // do not increment client index because we arent putting this one back

                                continue 'client_loop;
                            },
                            
                            _ => todo!("unhandled websocket message read error: {}", error.to_string())
                        }
                    },
                };
                let update_bytes = decompress_size_prepended(&compressed_update_bytes).expect("Failed to decompress update bytes");
    
                let update: Update = match bitcode::deserialize(&update_bytes) {
                    Ok(state_diff) => state_diff,
                    Err(error) => {
                        todo!("unhandled game state diff deserialization error: {}", error);
                    },
                };
    
                // relay this update to other clients
                'relay: for other_client_index in 0..self.clients.len() {
    
                    let mut other_client = self.clients.remove(other_client_index);
                    
                    // we keep on trying to send until the socket doesn't block
                    'send_attempt: loop {
                        match other_client.send(Message::Binary(compressed_update_bytes.clone())) {
                            Ok(_) => {
                                self.clients.insert(other_client_index, other_client);
    
                                continue 'relay;
    
                            },
    
                            //not yet implemented: unhandled error when relaying update data to client: IO error: A non-blocking socket operation could not be completed immediately. (os error 10035)
                            Err(error) => {
    
                                match error {
                                    tungstenite::Error::ConnectionClosed => todo!(),
                                    tungstenite::Error::AlreadyClosed => todo!(),
                                    tungstenite::Error::Io(io_error) => {
                                        match io_error.kind() {
  
                                            std::io::ErrorKind::WouldBlock => {
                                                println!("would've blocked!");
                                                continue 'send_attempt;
                                            },
            
                                            _ => todo!("unhandled io error when relaying update data to client: {}", io_error),
                                        }
                                    },
                                    _ => {todo!("unhandled error when relaying update data to client: {}", error)}
                                }
                                
        
                            },
                        }
                    }
                    
                    
    
                }

                // apply it to our own game state
                self.state.apply(&state_diff);
            }
        }
    }

    pub fn reset_level_if_no_players(&mut self) {

        if self.player_count == 0 {
             self.game_state = GameState::empty();
        }
        
    }

    pub fn run(&mut self) {

        loop {

            self.game_state.server_tick();

            self.reset_level_if_no_players();
            
            

            // slow the loop down a bit so that it doesnt use so much cpu
            std::thread::sleep(web_time::Duration::from_micros(200));
            
        }
    }


}

