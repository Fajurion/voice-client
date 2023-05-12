use opus::Encoder;

pub const SAMPLE_RATE: u32 = 48000;

pub fn encode(samples: Vec<f32>, channels: usize, encoder: &mut opus::Encoder) -> Vec<u8> {

    let mut output: Vec<u8> = vec![0u8; 3000];

    return match encoder.encode_float(&samples, &mut output) {
        Ok(size) => {
            output.split_at(size).0.to_vec()
        },
        Err(err) => panic!("error encoding: {}", err)
    };
}