use std::sync::Mutex;

use once_cell::sync::Lazy;
use rand::{Rng, rngs::ThreadRng};

pub mod udp;
pub mod auth;
pub mod receiver;

pub enum PrefixKind {
    Client,
    Encrypted
}

impl PrefixKind {
    fn byte(&self) -> u8 {
        match self {
            PrefixKind::Client => b'c',
            PrefixKind::Encrypted => b'e'
        }
    }
}

static CONN_ID: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| {
    let mut rng = rand::thread_rng();
    Mutex::new(vec![random_letter_byte(&mut rng), random_letter_byte(&mut rng), random_letter_byte(&mut rng), random_letter_byte(&mut rng)])
});

fn random_letter_byte(rng: &mut ThreadRng) -> u8 {
    rng.sample(rand::distributions::Alphanumeric) as u8
}

pub fn prefix_message(prefix: PrefixKind, data: &mut Vec<u8>) -> Vec<u8> {
    let mut prefix: Vec<u8> = vec![prefix.byte()];
    prefix.append(&mut CONN_ID.lock().unwrap().clone());
    prefix.append(&mut vec![b':']);
    prefix.append(data);
    prefix
}