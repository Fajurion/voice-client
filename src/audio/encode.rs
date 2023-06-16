use std::{thread, sync::{Mutex, mpsc::{Sender, Receiver, self}}};

use once_cell::sync::Lazy;

use crate::{connection::{self, auth}};

pub const SAMPLE_RATE: u32 = 48000;
pub const FRAME_SIZE: usize = 960;
 
pub fn encode(samples: Vec<f32>, encoder: &mut opus::Encoder) -> Vec<u8> {

    let mut output: Vec<u8> = vec![0u8; 3000];

    return match encoder.encode_float(&samples, &mut output) {
        Ok(size) => {
            output.split_at(size).0.to_vec()
        },
        Err(err) => panic!("error encoding: {}", err)
    };
}

static ENCODE_SENDER: Lazy<Mutex<Sender<Vec<f32>>>> = Lazy::new(|| {
    let (sender, _) = mpsc::channel();
    Mutex::new(sender)
});

static ENCODE_RECEIVER: Lazy<Mutex<Receiver<Vec<f32>>>> = Lazy::new(|| {
    let (_, receiver) = mpsc::channel();
    Mutex::new(receiver)
});

pub fn pass_to_encode(data: Vec<f32>) {
    ENCODE_SENDER.lock().expect("channel kaputt").send(data).expect("sending kaputt");
}

pub fn encode_thread(channels: usize) {

    // Encode channel
    let (sender, receiver) = mpsc::channel();
    let mut actual_sender = ENCODE_SENDER.lock().unwrap();
    *actual_sender = sender;
    let mut actual_receiver = ENCODE_RECEIVER.lock().unwrap();
    *actual_receiver = receiver;

    let opus_channel = match channels {
        1 => opus::Channels::Mono,
        2 => opus::Channels::Stereo,
        _ => panic!("invalid channel count")
    };

    // Spawn a thread
    thread::spawn(move || {

        // Create encoder
        let mut encoder = opus::Encoder::new(SAMPLE_RATE, opus_channel, opus::Application::Voip).unwrap();

        // Set the bitrate to 128 kb/s
        encoder.set_bitrate(opus::Bitrate::Bits(128000)).unwrap();

        loop {
            let samples = ENCODE_RECEIVER.lock().unwrap().recv().expect("Encoding channel broke");
            let samples_len = samples.len();

            if samples_len < FRAME_SIZE * channels {
                continue;
            }

            // TODO: Check if talking

            if auth::get_connection() {
                let mut encoded = encode(samples, &mut encoder);
                let mut channel = vec![b'v', b':'];
                channel.append(&mut encoded);    
                connection::udp::send(auth::encrypted_packet(&mut channel));
            }

            // Decode (FUTURE)
            //let decoded = decode::decode(encoded, FRAME_SIZE * channels, &mut decoder);
        }
    });
}