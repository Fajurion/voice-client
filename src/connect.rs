use std::net::UdpSocket;

use serde_json::json;

use crate::util;

pub fn connect() {

    let key = util::hasher::sha256("test".as_bytes().to_vec());

    let plaintext: &[u8] = b"Hurensohn im Toaster mit Marmelade und Butter";

    let ciphertext = util::crypto::encrypt(&key, plaintext);
    let decrypted = util::crypto::decrypt(&key, &ciphertext);

    println!("Plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("Ciphertext: {}", String::from_utf8_lossy(&ciphertext));
    println!("Plaintext: {}", String::from_utf8_lossy(&decrypted));

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
    let mut prefix: Vec<u8> = vec![b't', b':'];
    prefix.append(&mut ciphertext.to_vec());
    match socket.send(&prefix) {
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