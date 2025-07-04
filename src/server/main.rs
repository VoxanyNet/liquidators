use std::{net::SocketAddr, str::FromStr};

use liquidators_lib::server::Server;

fn main () {

    let mut server = Server::new(
        SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5556)
    );
    
    server.run();
}

