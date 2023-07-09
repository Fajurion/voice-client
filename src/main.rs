pub mod connection;
pub mod audio;
pub mod communication;
pub mod util;

static TESTING: bool = true;
static TESTING_ADDRESS: &str = "37.114.61.190:3006";

fn main() {  

    audio::microphone::record();

    if TESTING {
        connection::udp::connect(TESTING_ADDRESS);
    }
}