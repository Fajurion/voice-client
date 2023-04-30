use std::{time::Duration, thread};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, StreamConfig};
use opus::Encoder;
use rubato::{FftFixedOut, Resampler, FftFixedIn};

pub fn record_microphone() {
    
    // Get a cpal host
    let host = cpal::default_host(); // Current host on computer
    let device = host.default_input_device().expect("no input device available"); // Current device

    // Create a stream config
    let default_config = device.default_input_config().expect("no stream config found");
    let opus_sample_rate = 48000;
    let sample_rate = default_config.sample_rate().0;
    let channels = default_config.channels();

    // Create custom config with better buffer size
    let config = StreamConfig {
        channels: channels,
        sample_rate: cpal::SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Fixed(4096),
    };

    // Create opus encoder
    let opus_channel = match channels {
        1 => opus::Channels::Mono,
        2 => opus::Channels::Stereo,
        _ => panic!("unsupported channel count")
    };

    let mut encoder = Encoder::new(opus_sample_rate, opus_channel, opus::Application::Voip).unwrap();

    // Create a resampler
    let mut resampler = FftFixedOut::<f64>::new(
        sample_rate as usize,
        opus_sample_rate as usize,
        1024,
        1024,
        channels as usize,
    ).unwrap();

    // Create buffer for overflowing samples
    let mut overflow_buffer = Vec::<f32>::new();

    let mut sample_vec = Vec::<f32>::new();

    // Create a stream
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {

            // Cut down data and put rest into overflow buffer
            let sample: &[f32];

            let max_length = resampler.input_frames_next()*channels as usize;
            if data.len() > max_length {
                let (buffer, overflow) = data.split_at(max_length);
                overflow_buffer.extend_from_slice(overflow);
                sample = buffer;
            } else {
                sample = data;
            }
            // Convert into seperate channels
            let channels = extract_channels(sample, channels as usize);

            println!("{:?} channels: {:?}", channels.len(), data.len());
            println!("samples in 0: {:?}", channels[0].len());
            println!("samples in 1: {:?}", channels[1].len());

            // Resample
            let resampled = resampler.process(&channels, None).unwrap();
            let sample = reverse_extract_channels_2(&resampled); 

            // TODO: Save in large buffer and encode with opus every 20ms

            // Encode the samples using Opus
            /* 
            let mut encoded = [0u8; 2048];
            let len = match encoder.encode(&data, &mut encoded) {
                Ok(len) => len,
                Err(err) => panic!("error encoding: {}", err),
            };
            let encoded = &encoded[..len]; */

            // Add all to sample_vec
            sample_vec.extend_from_slice(&sample);

            println!("samples: {:?}", &sample_vec.len());

            // TODO: Do something
        },
        move |err| {
            eprintln!("an error occurred on stream: {}", err);
        },
        None,
    ).unwrap();

    // Play the stream
    stream.play().unwrap();

    // Pause after 10s
    thread::sleep(Duration::from_secs(1));

    stream.pause().unwrap();    
    
}

fn extract_channels(data: &[f32], channels: usize) -> Vec<Vec<f64>> {

    let mut channels_data = vec![Vec::<f64>::with_capacity(data.len() / channels); channels];
    for (_, sample) in data.chunks(channels).enumerate() {
        channels_data[0].push(sample[0] as f64);
        channels_data[1].push(sample[1] as f64);
    }
    channels_data
}

fn reverse_extract_channels_2(channels_data: &[Vec<f64>]) -> Vec<f32> {
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

