use std::{time::Duration, thread, sync::{Mutex, Arc}};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, StreamConfig};
use rubato::{FftFixedOut, Resampler};

use crate::audio::{encode};

use super::playback;

pub fn record() {
    
    // Get a cpal host
    let host = cpal::default_host(); // Current host on computer
    let device = host.default_input_device().expect("no input device available"); // Current device

    // Create a stream config
    let default_config = device.default_input_config().expect("no stream config found");
    let sample_rate = default_config.sample_rate().0;
    let channels = 1; // Stereo doesn't work at the moment (will fix in the future or never)

    // Create custom config with better buffer size
    let config = StreamConfig {
        channels: channels,
        sample_rate: cpal::SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Fixed(4096),
    };
    // Create a resampler
    let mut resampler = FftFixedOut::<f64>::new(
        sample_rate as usize,
        encode::SAMPLE_RATE as usize,
        encode::FRAME_SIZE,
        encode::FRAME_SIZE,
        channels as usize,
    ).unwrap();

    // Create a decoder with the same parameters as the encoder

    // Create buffer for overflowing samplesd
    let overflow_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let overflow_buffer_2: Arc<Mutex<Vec<f32>>> = overflow_buffer.clone();

    let sample_vec: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::<f32>::new()));
    let encoding_vec = sample_vec.clone();
    let recording_vec = sample_vec.clone();

    let decoded_samples: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::<f32>::new()));

    encode::encode_thread((channels as usize).clone(), encoding_vec, decoded_samples.clone());

    // Create a stream
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            resample_data(data, &mut resampler, &overflow_buffer_2, &recording_vec, channels);
        },
        move |err| {
            eprintln!("an error occurred on stream: {}", err);
        },
        None,
    ).unwrap();

    // Play the stream
    stream.play().unwrap();

    // Pause after 10s
    thread::sleep(Duration::from_secs(3));

    stream.pause().unwrap();

    // Play the audio
    playback::play_audio(decoded_samples.lock().unwrap().clone(), channels as usize);

}

fn resample_data(
    data: &[f32], 
    resampler: &mut FftFixedOut<f64>, 
    overflow_buffer_p: &Arc<Mutex<Vec<f32>>>, 
    sample_vec: &Arc<Mutex<Vec<f32>>>, 
    channels: u16,
) {

    // Cut down to needed size and add to overflow buffer
    let mut max_length = resampler.input_frames_next()*channels as usize;
    println!("data: {}", data.len());
    println!("max_length: {}", max_length);
    let mut overflow_buffer = overflow_buffer_p.lock().unwrap();
    overflow_buffer.extend_from_slice(data);

    while overflow_buffer.len() > max_length {

        // Get data to resample from overflow
        let buffer = if overflow_buffer.len() > max_length {
            let buffer = overflow_buffer.drain(0..max_length).collect::<Vec<f32>>();
            buffer
        } else {
            return;
        };
        
        // Extract channels
        let extracted_channels = extract_channels(&buffer, channels as usize);

        // Resample and reverse extract
        let resampled = resampler.process(&extracted_channels, None).unwrap();
        let sample = reverse_extract_channels(&resampled);

        // Add all to sample_vec
        println!("sample: {:?}", sample.len());
        sample_vec.lock().unwrap().extend_from_slice(&sample);

        // Prepare for next iteration
        max_length = resampler.input_frames_next()*channels as usize;

        /* OLD CODE
        // Encode with opus
        let samples = encode::encode(sample, encoder);

        println!("encoded: {:?}", samples.len()); 

        // Decode for testing
        let decoded_samples = decode::decode(samples, len as usize, decoder); */
    }
}

fn extract_channels(data: &[f32], channels: usize) -> Vec<Vec<f64>> {

    let mut channels_data = vec![Vec::<f64>::with_capacity(data.len() / channels); channels];
    for (_, sample) in data.chunks(channels).enumerate() {
        channels_data[0].push(sample[0] as f64);
        if channels == 2 {
            channels_data[1].push(sample[1] as f64);
        }
    }
    channels_data
}

// Thanks ChatGPT
fn reverse_extract_channels(channels_data: &[Vec<f64>]) -> Vec<f32> {
    let channel_size = channels_data.len();
    let num_samples = channels_data[0].len();
    let mut interleaved_samples = Vec::with_capacity(num_samples * channel_size);
    for i in 0..num_samples {
        for j in 0..channel_size {
            interleaved_samples.push(channels_data[j][i] as f32);
        }
    }
    interleaved_samples
}

