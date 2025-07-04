use std::{net::SocketAddr, str::FromStr};

use liquidators_lib::editor_server::EditorServer;

fn main() {
    let address = SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5557);

    let mut server = EditorServer::new(address);

    server.run();
}