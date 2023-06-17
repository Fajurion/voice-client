pub mod connection;
pub mod audio;
pub mod communication;
pub mod util;

static TESTING: bool = true;
static TESTING_ADDRESS: &str = "localhost:3011";

fn main() {  

    audio::microphone::record();

    if TESTING {
        connection::udp::connect(TESTING_ADDRESS);
    }
}