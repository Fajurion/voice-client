use std::net::UdpSocket;

use crate::util;

use super::auth::{test_auth_packet, turn_into_packet};

pub fn connect(address: &str) {

    // Bind to a local address
    let socket = match UdpSocket::bind("localhost:3422") {
        Ok(s) => s,
        Err(_) => {
            println!("Could not bind socket");
            return;
        }
    };

    // Connect to a remote address
    match socket.connect(address) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not connect");
            return; 
        }
    }

    // Send auth packet
    let mut packet = test_auth_packet(util::random_string(6));
    let packaged = turn_into_packet(&mut packet);

    match socket.send(packaged.as_slice()) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not send"); 
            return;
        }
    }

}