use std::net::{SocketAddr, TcpListener, TcpStream};

use diff::Diff;
use liquidators_lib::game_state::{GameState, GameStateDiff};
use gamelibrary::traits::HasOwner;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use tungstenite::{Message, WebSocket};

pub struct Server {
    pub listener: TcpListener,
    pub clients: Vec<WebSocket<TcpStream>>,
    pub game_state: GameState,
    pub update_history: Vec<GameState>
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {

        let listener = match TcpListener::bind(address) {
            Ok(listener) => listener,
            Err(error) => panic!("failed to bind listener: {}", error),
        };

        match listener.set_nonblocking(true) {
            Ok(_) => {},
            Err(error) => panic!("failed to set server as non blocking: {}", error),
        };

        Self {
            listener,
            clients: vec![],
            game_state: GameState::empty(),
            update_history: vec![],
        }

        
    }

    pub fn run(&mut self) {

        loop {
            
            self.accept_new_client();

            self.receive_updates();

            // slow the loop down a bit so that it doesnt use so much cpu
            //std::thread::sleep(Duration::from_millis(1));
            
        }
    }

    pub fn disconnect_client(&mut self, client_uuid: String) {
        for entity_index in 0..self.game_state.entities.len() {
            if self.game_state.entities[entity_index].get_owner() == client_uuid {
                self.game_state.entities.remove(entity_index);
            }
        }
    }

    pub fn receive_updates(&mut self) {

        'client_loop: for client_index in 0..self.clients.len() {

            // take the client out, receive all updates, then put it back in
            let mut client = self.clients.remove(client_index);
            
            // keep trying to receive updates until there are none
            loop {

                let compressed_game_state_diff_string_bytes = match client.read() {
                    Ok(message) => {
                        match message {
                            Message::Binary(game_state_diff_bytes) => {
                                game_state_diff_bytes
                            },
                            _ => todo!("client tried to send non binary message")
                        }
                    },
                    Err(error) => {
                        match error {

                            tungstenite::Error::Io(io_error) => {
                                match io_error.kind() {
                                    std::io::ErrorKind::WouldBlock => {
                                        // this means that there was no update to read
                                        self.clients.insert(client_index, client);
                                        
                                        continue 'client_loop // move to the next client
                                    },
                                    _ => todo!("unhandled io error: {}", io_error),
                                }
                            },
                            _ => todo!("unhandled websocket message read error: {}", error)
                        }
                    },
                };
                let game_state_diff_string_bytes = decompress_size_prepended(&compressed_game_state_diff_string_bytes).expect("Failed to decompress game state diff string bytes");

                let game_state_diff_string = match String::from_utf8(game_state_diff_string_bytes.clone()) {
                    Ok(game_state_diff_string) => game_state_diff_string,
                    Err(error) => {
                        todo!("unhandled game state byte decoding error {}", error);
                    },
                };
    
                let game_state_diff: GameStateDiff = match serde_json::from_str(&game_state_diff_string) {
                    Ok(game_state_diff) => game_state_diff,
                    Err(error) => {
                        todo!("unhandled game state diff deserialization error: {}", error);
                    },
                };
    
                // relay this update to other clients
                'relay: for other_client_index in 0..self.clients.len() {
    
                    let mut other_client = self.clients.remove(other_client_index);
    
                    match other_client.send(Message::Binary(compressed_game_state_diff_string_bytes.clone())) {
                        Ok(_) => {
                            self.clients.insert(other_client_index, other_client);

                            continue 'relay;

                        },
                        Err(error) => {
                            todo!("unhandled error when relaying update data to client: {}", error);
    
                        },
                    }
    
                }

                // apply it to our own game state
                self.game_state.apply(&game_state_diff);
            }
        }
    }

    pub fn accept_new_client(&mut self) -> Option<()>{

        match self.listener.accept() {
            Ok((stream, address)) => {
                println!("received new connection from address: {}", address);

                stream.set_nonblocking(true).expect("Failed to set new client as non blocking");

                let mut websocket_stream = loop {
                    match tungstenite::accept(stream.try_clone().expect("failed to clone stream")) {
                        Ok(websocket_stream) => break websocket_stream,
                        Err(error) => {
                            match error {
                                tungstenite::HandshakeError::Interrupted(_) => continue, // try again if the handshake isnt done yet
                                tungstenite::HandshakeError::Failure(error) => panic!("handshake failed with new client: {}", error),
                            }
                        },
                    };
                };
                

                // send client current state
                let game_state_string = serde_json::to_string(&self.game_state).expect("Failed to serialize current game state");

                let game_state_bytes = game_state_string.as_bytes().to_vec();

                let compressed_game_state_bytes = compress_prepend_size(&game_state_bytes);

                // keep attempting to send initial state to client
                loop {
                    match websocket_stream.send(
                        Message::Binary(compressed_game_state_bytes.clone())
                    ) {
                        Ok(_) => break,
                        Err(error) => {
                            match error {
                                tungstenite::Error::Io(io_error) => {
                                    match io_error.kind() {
                                        std::io::ErrorKind::WouldBlock => {
                                            continue; // try again if the socket blocked
                                        },
                                        _ => panic!("Something went wrong trying to send initial state: {}", io_error)
                                    }
                                },
                                _ => panic!("Something went wrong trying to send initial state: {}", error)
                            }
                        },
                    }
                }

                println!("pushing new client");

                self.clients.push(websocket_stream);

                return Some(())

            },
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => return None, // no new clients

                    _ => {
                        println!("Something went wrong trying to accept a new client");
                        return None
                    }
                }
            },
        }
    }



}

