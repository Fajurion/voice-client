use std::time::Duration;
use std::{io, thread};
use std::net::UdpSocket;

use crate::connection::receiver;
use crate::{connection::{auth, self}, util};

pub fn connect(address: &str) {
    connect_recursive(address, 0)
}

fn connect_recursive(address: &str, tries: u8) {

    if tries > 5 {
        println!("Could not connect");
        return;
    }

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

    // Wait for confirmation
    let mut buf: [u8; 8192] = [0u8; 8192];
    let mut authenticated = false;
    let mut confirm_tries = 0;

    println!("Waiting for confirmation..");

    while !authenticated {
        if confirm_tries > 5 {
            println!("Could not authenticate");
            return;
        }

        // TODO: Add a timeout in case of a packet loss
        let size = match socket.recv(&mut buf) {
            Ok(size) => size,
            Err(_) => 0
        };

        if size == 0 {
            println!("Could not receive: Retrying..");
            connect_recursive(address, tries + 1);
            return;
        }

        let decrypted = util::crypto::decrypt(&auth::get_encryption_key(), &buf[0..size]);
        let string = String::from_utf8_lossy(&decrypted);


        if string == input {
            println!("Authenticated");
            authenticated = true;
            continue;
        }

        match socket.send(&auth::confirm_packet()) {
            Ok(_) => {}
            Err(_) => {
                println!("Could not send"); 
                return;
            }
        }

        confirm_tries += 1;

        thread::sleep(Duration::from_secs(1));
    }

    // Start threads
    auth::refresh_thread(&socket);

    // Listen for udp traffic
    let mut buf: [u8; 8192] = [0u8; 8192];
    loop {

        let size = socket.recv(&mut buf).expect("Detected disconnect");
        match receiver::receive_packet(&buf[0..size].to_vec()) {
            Ok(_) => (),
            Err(message) => println!("{}", message)
        }
    }    

}