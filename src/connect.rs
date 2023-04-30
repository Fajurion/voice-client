pub fn connect() {
    // Bind to a local address
    let socket = match UdpSocket::bind("localhost:3422") {
        Ok(s) => s,
        Err(_) => {
            println!("Could not bind socket");
            return;
        }
    };

    // Connect to a remote address
    match socket.connect("localhost:3006") {
        Ok(_) => {}
        Err(_) => {
            println!("Could not connect");
            return; 
        }
    }

    // Send test message
    match socket.send("hi friends".as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not send");
            return;
        }
    }
}