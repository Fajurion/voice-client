use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::time::Duration;
use std::{io, thread};
use std::net::UdpSocket;

use once_cell::sync::Lazy;
use rand::Rng;

use crate::audio;
use crate::connection::receiver;
use crate::communication;
use crate::{connection::{auth, self}, util};

static SEND_SENDER: Lazy<Mutex<Sender<Vec<u8>>>> = Lazy::new(|| {
    let (sender, _) = mpsc::channel();
    Mutex::new(sender)
});

static SEND_RECEIVER: Lazy<Mutex<Receiver<Vec<u8>>>> = Lazy::new(|| {
    let (_, receiver) = mpsc::channel();
    Mutex::new(receiver)
});

pub fn send(data: Vec<u8>) {
    SEND_SENDER.lock().expect("channel broken").send(data).expect("sending broken");
}

pub fn connect(address: &str) {

    // Sending channel
    let (sender, receiver) = mpsc::channel();
    let mut actual_sender = SEND_SENDER.lock().unwrap();
    *actual_sender = sender;
    let mut actual_receiver = SEND_RECEIVER.lock().unwrap();
    *actual_receiver = receiver;

    drop(actual_sender);
    drop(actual_receiver);

    connect_recursive(address, 0);
}

fn connect_recursive(address: &str, tries: u8) {

    if tries > 5 {
        util::print_log("Could not connect");
        return;
    }

    // Bind to a local address
    let socket = match UdpSocket::bind(format!("localhost:{}", rand::thread_rng().gen_range(3000..4000))) {
        Ok(s) => s,
        Err(_) => {
            util::print_log("Could not bind socket");
            return;
        }
    };

    // Connect to a remote address
    match socket.connect(address) {
        Ok(_) => {}
        Err(_) => {
            util::print_log("Could not connect to remote address");
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
    util::print_log(format!("{}", String::from_utf8_lossy(&packet)).as_str());

    let packaged = connection::prefix_message(auth::get_auth_prefix(), &mut packet);

    match socket.send(packaged.as_slice()) {
        Ok(_) => {}
        Err(_) => {
            util::print_log("Could not send"); 
            return;
        }
    }

    // Wait for confirmation
    let mut buf: [u8; 8192] = [0u8; 8192];
    let mut authenticated = false;
    let mut confirm_tries = 0;

    util::print_log("Waiting for confirmation..");

    while !authenticated {
        if confirm_tries > 5 {
            util::print_log("Could not authenticate");
            return;
        }

        // TODO: Add a timeout in case of a packet loss
        let size = match socket.recv(&mut buf) {
            Ok(size) => size,
            Err(_) => 0
        };

        if size == 0 {
            util::print_log("Could not receive: Retrying..");
            connect_recursive(address, tries + 1);
            return;
        }

        let decrypted = util::crypto::decrypt(&auth::get_encryption_key(), &buf[0..size]);
        let string = String::from_utf8_lossy(&decrypted);

        if string == input {
            util::print_log("Authenticated");
            authenticated = true;
            continue;
        }

        match socket.send(&auth::confirm_packet()) {
            Ok(_) => {}
            Err(_) => {
                util::print_log("Could not send"); 
                return;
            }
        }

        confirm_tries += 1;

        thread::sleep(Duration::from_secs(1));
    }

    // Start threads
    auth::refresh_thread(&socket);
    send_thread(socket.try_clone().expect("Could not clone socket"));
    audio::decode::decode_play_thread();

    auth::set_connection(true);

    // Listen for udp traffic
    thread::spawn(move || {
        let mut buf: [u8; 8192] = [0u8; 8192];
        loop {
            let size = socket.recv(&mut buf).expect("Detected disconnect");

            match receiver::receive_packet(&buf[0..size].to_vec()) {
                Ok(_) => (),
                Err(message) => util::print_log(format!("{}", message).as_str())
            }
        } 
    });

    communication::start_listening();
}

// Starts a thread that sends data from the sender channel
fn send_thread(socket: UdpSocket) {

    thread::spawn(move || {
        loop {
            let data = SEND_RECEIVER.lock().expect("brokey").recv().expect("Packet sending channel broke");

            match socket.send(&data) {
                Ok(_) => {}
                Err(_) => {
                    util::print_log("Could not send"); 
                    return;
                }
            }
        }
    });
}