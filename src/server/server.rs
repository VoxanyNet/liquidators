use std::{io::{Error, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, time::Duration};

use diff::Diff;
use game::{game_state::{GameState, GameStateDiff}, networking::{receive_headered, send_headered}};
use macroquad::{color::WHITE, time::get_fps};

pub struct Server {
    pub listener: TcpListener,
    pub clients: Vec<TcpStream>,
    pub game_state: GameState,
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
        }

        
    }

    pub fn run(&mut self) {

        loop {
            
            self.accept_new_client();

            self.receive_updates();

            // slow the loop down a bit so that it doesnt use so much cpu
            std::thread::sleep(Duration::from_micros(100));
            
        }
    }

    pub fn receive_updates(&mut self) {

        for client_index in 0..self.clients.len() {

            // take the client out, receive updates, then put it back in
            let mut client = self.clients.swap_remove(client_index);
            
            let mut game_state_diff_string_bytes = match receive_headered(&mut client) {
                Ok(game_state_diff_string_bytes) => {
                    println!("Received an update from a client!");
                    game_state_diff_string_bytes
                },
                Err(error) => {
                    match error.kind() {
                        std::io::ErrorKind::WouldBlock => {
                            //println!("skipping client because they would have blocked");

                            self.clients.push(client);

                        },
                        _ => {
                            println!("something went wrong trying to receive update from client: {}", error);
                            
                            // DONT put client back in. we want to disconnect them
                            self.clients.push(client);
                        }
                    }

                    continue; // skip to next client if fail
                },
            };
            
            let game_state_diff_string = match String::from_utf8(game_state_diff_string_bytes.clone()) {
                Ok(game_state_diff_string) => game_state_diff_string,
                Err(error) => {
                    println!("failed to decode game state diff as string {}", error);

                    self.clients.push(client);
                    continue;
                },
            };

            println!("{}", game_state_diff_string);

            let game_state_diff: GameStateDiff = match serde_json::from_str(&game_state_diff_string) {
                Ok(game_state_diff) => game_state_diff,
                Err(error) => {
                    println!("failed to deserialize game state diff: {}", error);

                    self.clients.push(client);

                    continue;
                },
            };

            // relay this update to other clients
            for other_client_index in 0..self.clients.len() {

                let mut other_client = self.clients.swap_remove(other_client_index);

                match send_headered(game_state_diff_string_bytes.as_mut_slice(), &mut other_client) {
                    Ok(_) => {},
                    Err(error) => {
                        println!("failed to relay update data to client: {}", error);

                        self.clients.push(other_client);

                        continue;

                    },
                }

                self.clients.push(other_client);
            }
            
            // apply it to our own game state
            self.game_state.apply(&game_state_diff);

            self.clients.push(client);

        }
    }

    pub fn accept_new_client(&mut self) -> Option<()>{

        match self.listener.accept() {
            Ok((mut stream, address)) => {
                println!("received new connection from address: {}", address);

                // send client current state
                let game_state_string = match serde_json::to_string(&self.game_state) {
                    Ok(game_state_bytes) => game_state_bytes,
                    Err(error) => {
                        println!("failed serialize initial state to string: {}", error);
                        return None
                    },
                };

                let game_state_bytes = game_state_string.as_bytes();

                match send_headered(game_state_bytes, &mut stream) {
                    Ok(_) => {},
                    Err(error) => {
                        println!("failed to send initial state: {}", error);
                        return None
                    }
                }
                
                // only set as non blocking once the initial state has been sent
                match stream.set_nonblocking(true) {
                    Ok(_) => {},
                    Err(error) => {
                        println!("failed to set new client as non blocking: {}", error);
                        return None
                    },
                }
                
                println!("pushing new client");

                self.clients.push(stream);

                Some(())

            },
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => None, //  no new clients

                    _ => None
                }
            },
        }
    }



}

