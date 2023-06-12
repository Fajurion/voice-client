pub mod udp;
pub mod auth;

pub enum PrefixKind {
    Testing,
    Client
}

impl PrefixKind {
    fn byte(&self) -> u8 {
        match self {
            PrefixKind::Testing => b't',
            PrefixKind::Client => b'c'
        }
    }
}

pub fn prefix_message(prefix: PrefixKind) -> Vec<u8> {
    let mut prefix: Vec<u8> = vec![prefix.byte(), b':'];
    prefix.append(&mut "test".as_bytes().to_vec());
    prefix
}