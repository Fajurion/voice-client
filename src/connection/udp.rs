use std::io;
use std::net::UdpSocket;

use crate::connection::{auth, self};

pub fn connect(address: &str) {

    // Bind to a local address
    let socket = match UdpSocket::bind("localhost:3422") {
        Ok(s) => s,
        Err(_) => {
            println!("Could not bind socket");
            return;
        }
    };

    // Connect to a remote address
    match socket.connect(address) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not connect");
            return; 
        }
    }

    // Ask for id
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();

    auth::set_encryption_key(auth::get_key(&input));

    // Send auth packet (ONLY TESTING)
    let mut packet = auth::auth_packet(&input, &input, &input, "test");

    // Turn into string and print
    println!("{}", String::from_utf8_lossy(&packet));

    let packaged = connection::prefix_message(auth::get_auth_prefix(), &mut packet);

    match socket.send(packaged.as_slice()) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not send"); 
            return;
        }
    }

    // Start threads
    auth::refresh_thread(&socket);

    // Listen for udp traffic
    let mut buf = [0u8; 8192];
    loop {

        socket.recv(&mut buf).expect("Detected disconnect");

        let string = String::from_utf8_lossy(&buf);
        println!("{}", string)
    }    

}