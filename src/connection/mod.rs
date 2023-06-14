pub mod udp;
pub mod auth;

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

pub fn prefix_message(prefix: PrefixKind, data: &mut Vec<u8>) -> Vec<u8> {
    let mut prefix: Vec<u8> = vec![prefix.byte(), b':'];
    prefix.append(data);
    prefix
}