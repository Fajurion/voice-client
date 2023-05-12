pub fn decode(samples: Vec<u8>, channels: usize, buffer_size: usize, decoder: &mut opus::Decoder) -> Vec<f32> {

    let mut output: Vec<f32> = vec![0f32; buffer_size];

    return match decoder.decode_float(&samples, &mut output, false) {
        Ok(size) => {
            output.split_at(size).0.to_vec()
        },
        Err(err) => panic!("error decoding: {}", err)
    };
}