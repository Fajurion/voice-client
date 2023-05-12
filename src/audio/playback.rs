use rodio::{OutputStream, Sink, buffer::SamplesBuffer};

use super::encode;

pub fn play_audio(samples: Vec<f32>, channels: usize) {

    let source = SamplesBuffer::new(channels as u16, encode::SAMPLE_RATE as u32, samples);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    sink.sleep_until_end();

    /*
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    stream_handle.play_raw(&samples); */

}