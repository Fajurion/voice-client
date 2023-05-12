use std::{thread, time::Duration, sync::{Mutex, Arc}};

use crate::audio::decode;

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

pub fn encode_thread(channels: usize, samples: Arc<Mutex<Vec<f32>>>, decoded_samples: Arc<Mutex<Vec<f32>>>) {

    let opus_channel = match channels {
        1 => opus::Channels::Mono,
        2 => opus::Channels::Stereo,
        _ => panic!("invalid channel count")
    };

    // Create encoder
    let mut encoder = opus::Encoder::new(SAMPLE_RATE, opus_channel, opus::Application::Voip).unwrap();

    // Set the bitrate to 128 kb/s
    encoder.set_bitrate(opus::Bitrate::Bits(128000)).unwrap();

    let mut decoder = opus::Decoder::new(SAMPLE_RATE, opus_channel).unwrap();

    // Spawn a thread
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(20));

            let len = samples.lock().unwrap().len();

            if len < FRAME_SIZE * channels {
                println!("not enough samples | len: {}", len);
                continue;
            }

            println!("len: {}", len);

            let other_samp: Vec<f32> = samples.lock().unwrap().drain(0..FRAME_SIZE * channels).collect();


            // Cut off rest

            // Check if talking

            let encoded = encode(other_samp, &mut encoder);

            // Decode
            let decoded = decode::decode(encoded, FRAME_SIZE * channels, &mut decoder);
            decoded_samples.lock().unwrap().extend_from_slice(&decoded);

            // TODO: Send
        }
    });
}