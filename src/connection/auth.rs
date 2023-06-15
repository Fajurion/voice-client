use std::sync::Mutex;
use std::{net::UdpSocket, thread, time::Duration};

use base64::{engine::general_purpose, Engine};
use once_cell::sync::Lazy;

use crate::util;

use super::PrefixKind;

static ENCRYPTION_KEY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![])); 

pub fn set_encryption_key(key: Vec<u8>) {
    let mut encryption_key = ENCRYPTION_KEY.lock().unwrap();
    *encryption_key = key;
}

pub fn get_encryption_key() -> Vec<u8> {
    let encryption_key = ENCRYPTION_KEY.lock().unwrap();
    encryption_key.clone()
}

pub fn encrypted_packet(data: &mut Vec<u8>) -> Vec<u8> {

    // Encrypt data
    let mut encrypted = util::crypto::encrypt(&get_encryption_key(), data);

    let mut prefix: Vec<u8> = vec![get_prefix().byte(), b':'];
    prefix.append(&mut encrypted);
    prefix
}

pub fn auth_packet(id: &str, token: &str, secret: &str, room: &str) -> Vec<u8> {

    // Generate secret
    let encrypted_secret = util::crypto::encrypt(&get_key(token), secret.as_bytes());
    let mut encoded_secret = String::new();
    general_purpose::STANDARD_NO_PAD.encode_string(encrypted_secret, &mut encoded_secret);

    format!("{}:{}:{}", id, room, encoded_secret).into_bytes()
}

pub fn confirm_packet() -> Vec<u8> {
    encrypted_packet(&mut "c:c".as_bytes().to_vec())
}

pub fn get_auth_prefix() -> PrefixKind {
    PrefixKind::Client
}

pub fn get_prefix() -> PrefixKind {
    PrefixKind::Encrypted
}

pub fn get_key(token: &str) -> Vec<u8> {
    util::hasher::sha256(token.as_bytes().to_vec())
}

pub fn refresh_thread(socket: &UdpSocket) {

    let cloned = socket.try_clone().expect("Could not clone socket");
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(20));
    
        let packaged = encrypted_packet(&mut vec![b'r', b':', b'r']);
    
        match cloned.send(packaged.as_slice()) {
            Ok(_) => {}
            Err(_) => {
                println!("Could not send"); 
                return;
            }
        }
    });
}