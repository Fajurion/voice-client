mod connection;
mod audio;
mod util;

mod connect;

fn main() {   

    let key: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
    ];

    let plaintext: &[u8] = b"Hello world!";

    let ciphertext = util::crypto::encrypt(&key, plaintext);
    let decrypted = util::crypto::decrypt(&key, &ciphertext);

    println!("Plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("Ciphertext: {}", String::from_utf8_lossy(&ciphertext));
    println!("Plaintext: {}", String::from_utf8_lossy(&decrypted));

    connect::connect();

    //audio::microphone::record();
}