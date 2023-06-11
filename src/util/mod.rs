use rand::distributions::{Alphanumeric, DistString};

pub mod hasher;
pub mod crypto;

pub fn random_string(length: usize) -> String {
    return Alphanumeric.sample_string(&mut rand::thread_rng(), length);
}