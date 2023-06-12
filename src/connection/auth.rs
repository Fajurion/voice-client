use base64::{engine::general_purpose, Engine};
use serde_json::json;

use crate::util;

pub fn turn_into_packet(packet: &mut Vec<u8>) -> Vec<u8> {

    let mut prefix = vec![get_prefix().byte(), b':'];
    prefix.append(packet);

    prefix
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

    // Generate secret
    let encrypted_secret = util::crypto::encrypt(&get_key(), crate::TESTING_SECRET.as_bytes());
    let mut encoded_secret = String::new();
    general_purpose::STANDARD_NO_PAD.encode_string(encrypted_secret, &mut encoded_secret);

    let mut encoded_json = String::new();
    general_purpose::STANDARD.encode_string(packet.to_string(), &mut encoded_json);

    format!("{}:{}:{}", id, encoded_secret, encoded_json).into_bytes()
}

pub fn get_prefix() -> crate::connection::PrefixKind {
    if crate::TESTING {
        crate::connection::PrefixKind::Testing
    } else {
        crate::connection::PrefixKind::Client
    }
}

pub fn get_key() -> Vec<u8> {
    util::hasher::sha256(crate::TESTING_SECRET.as_bytes().to_vec())
}