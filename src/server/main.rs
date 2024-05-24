use std::{net::SocketAddr, str::FromStr};

use macroquad::{miniquad::conf::Platform, window::Conf};


pub mod server;

fn main () {

    let mut server = server::Server::new(
        SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5556)
    );
    
    server.run();
}

