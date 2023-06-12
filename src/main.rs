pub mod connection;
pub mod audio;
pub mod util;

static TESTING: bool = true;
static TESTING_ADDRESS: &str = "localhost:3011";
static TESTING_SECRET: &str = "deinemutter123";

fn main() {   

    if TESTING {
        connection::udp::connect(TESTING_ADDRESS);
    }

    //audio::microphone::record();
}