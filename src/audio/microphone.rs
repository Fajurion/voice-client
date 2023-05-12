use std::{time::Duration, thread, sync::{Mutex, Arc}};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, StreamConfig};
use opus::{Channels, Application};
use rubato::{FftFixedOut, Resampler};

use crate::audio::{encode, decode, playback};

pub fn record() {
    
    // Get a cpal host
    let host = cpal::default_host(); // Current host on computer
    let device = host.default_input_device().expect("no input device available"); // Current device

    // Create a stream config
    let default_config = device.default_input_config().expect("no stream config found");
    let sample_rate = default_config.sample_rate().0;
    let channels = 1;

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
        1920,
        1920,
        channels as usize,
    ).unwrap();

    // Create an encoder with 48 kHz sample rate, mono channel, and voice application
    let mut encoder = opus::Encoder::new(48000, Channels::Mono, Application::Voip).unwrap();
    encoder.set_bitrate(opus::Bitrate::Bits(128000)).unwrap(); // Set the bitrate to 32 kb/s

    // Create a decoder with the same parameters as the encoder
    let mut decoder = opus::Decoder::new(48000, Channels::Mono).unwrap();

    // Create buffer for overflowing samples
    let overflow_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let overflow_buffer_2 = overflow_buffer.clone();

    let mut sample_vec: Vec<f32> = Vec::<f32>::new();
    // Create a stream
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            resample_data(data, &mut resampler, &overflow_buffer_2, &mut sample_vec, channels, &mut encoder, &mut decoder);
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

}

fn resample_data(
    data: &[f32], 
    resampler: &mut FftFixedOut<f64>, 
    overflow_buffer_p: &Arc<Mutex<Vec<f32>>>, 
    sample_vec: &mut Vec<f32>, 
    channels: u16,
    encoder: &mut opus::Encoder,
    decoder: &mut opus::Decoder,
) {

    // Cut down to needed size and add to overflow buffer
    let mut max_length = resampler.input_frames_next()*channels as usize;
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

        println!("{:?} channels: {:?}", extracted_channels.len(), data.len());
        println!("samples in 0: {:?}/{:?}", extracted_channels[0].len(), max_length / 2);
        if channels == 2 {
            println!("samples in 1: {:?}/{:?}", extracted_channels[1].len(), max_length / 2);
        }
        println!("overflow: {:?}", overflow_buffer.len());

        // Resample and reverse extract
        let resampled = resampler.process(&extracted_channels, None).unwrap();
        let sample = reverse_extract_channels_2(&resampled);
        let len = sample.len();

        // Add all to sample_vec
        //sample_vec.extend_from_slice(&sample);

        // Prepare for next iteration
        max_length = resampler.input_frames_next()*channels as usize;

        println!("resampled: {:?}", sample.len());

        // Encode with opus
        let samples = encode::encode(sample, channels as usize, encoder);

        println!("encoded: {:?}", samples.len());

        // Decode for testing
        let decoded_samples = decode::decode(samples, channels as usize, len as usize, decoder);

        sample_vec.extend_from_slice(&decoded_samples);
    }

    println!("samples: {:?}", &sample_vec.len());

    // Play audio
    if sample_vec.len() > 130000 {
        playback::play_audio(sample_vec.clone(), channels as usize);
        panic!("hi")
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

