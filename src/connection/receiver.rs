use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util;

use super::auth;

#[derive(Deserialize, Serialize)]
struct Event {
    name: String,
    sender: String,
    data: Value
}

pub fn receive_packet(data: &Vec<u8>) -> Result<(), String> {

    let decrypted = util::crypto::decrypt(&auth::get_encryption_key(), data);
    println!("{}", String::from_utf8_lossy(decrypted.as_slice()));
    let event: Event = match serde_json::from_str(String::from_utf8_lossy(decrypted.as_slice()).as_ref()) {
        Ok(event) => event,
        Err(_) => return Err("Could not parse event".to_string())
    };

    println!("Received event: {}", event.name);

    return Ok(())
}