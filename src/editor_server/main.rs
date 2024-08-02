use std::{net::SocketAddr, str::FromStr};

use editor_server::EditorServer;

pub mod editor_server;

fn main() {
    let address = SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5557);

    let mut server = EditorServer::new(address);

    server.run();
}