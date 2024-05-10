use std::{net::SocketAddr, str::FromStr};

use game::collider::Collider;

pub mod server;

fn main () {
    let mut server = server::Server::new(
        SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").expect("failed to parse ip"), 5556)
    );

    let mut collider = rapier2d::geometry::ColliderBuilder::cuboid(300., 30.).build();

    let mut stop_it_now: Collider = collider.into();

    dbg!(stop_it_now.hx);
    
    server.run();
}

