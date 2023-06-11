use std::net::UdpSocket;

use serde_json::json;

use crate::util;

static testSecret: &str = "hi";

pub fn connect() {
    
    // Bind to a local address
    let socket = match UdpSocket::bind("localhost:3422") {
        Ok(s) => s,
        Err(_) => {
            println!("Could not bind socket");
            return;
        }
    };

    // Connect to a remote address
    match socket.connect("localhost:3011") {
        Ok(_) => {}
        Err(_) => {
            println!("Could not connect");
            return; 
        }
    }

    // Send test message
    match socket.send(test_auth_packet(util::random_string(4)).as_slice()) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not send");
            return;
        }
    }
}

pub fn test_auth_packet(id: String) -> Vec<u8> {
    // Unprotected very secret account details (totally my real account)
    let username = format!("tester {}", id);

    // Construct auth packet
    let packet = json!({
        "id": id,
        "username": username,
        "session": id,
        "tag": "test"
    });

    format!("c:{}:", packet.to_string()).into_bytes()
}