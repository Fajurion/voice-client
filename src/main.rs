pub mod connection;
pub mod audio;
pub mod communication;
pub mod util;

static TESTING: bool = false;
static TESTING_ADDRESS: &str = "localhost:3006";

fn main() {  

    audio::microphone::record();

    if TESTING {
        connection::udp::connect(TESTING_ADDRESS);
    } else {
        audio::decode::decode_play_thread();
        communication::start_listening();
    }
}