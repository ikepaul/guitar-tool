use crate::fft::fft;
use bevy::ecs::event::Event;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, SampleRate, StreamConfig,
};

use crate::constants::{BUFFER_SIZE, SAMPLE_RATE};
use std::sync::mpsc::Sender;

#[derive(Event)]
pub struct FreqEvent {
    pub heights: Vec<f32>,
}

pub fn mic_setup(tx: Sender<FreqEvent>) {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("no input device available");

    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let mut config: StreamConfig = supported_config.into();
    config.buffer_size = BufferSize::Fixed(BUFFER_SIZE);
    config.sample_rate = SampleRate(SAMPLE_RATE);
    tx.send(FreqEvent {
        heights: vec![1., 2., 3., 4., 5., 6.],
    })
    .expect("to be able to send");
    let lambda_data_callback =
        move |data: &[f32], x: &cpal::InputCallbackInfo| data_callback(data, x, &tx);

    let stream = device
        .build_input_stream(&config, lambda_data_callback, error_callback, None)
        .expect("Stream failed to build");

    println!("Starting stream! {:?}", config);
    stream.play().expect("Stream failed to play");
    std::thread::sleep(std::time::Duration::MAX);
    println!("WE ARE PLAYING");
}

fn data_callback(data: &[f32], _: &cpal::InputCallbackInfo, tx: &Sender<FreqEvent>) {
    /* let mut max_sample: f32 = 0.0;
    for &sample in data {
        max_sample = if max_sample > sample {
            max_sample
        } else {
            sample
        };
    }
     */
    /* let mut freqs: Vec<(usize, f32, f32)> = transformed
    .iter()
        .enumerate()
        .take(100)
        .map(|(i, &(a, b))| {
            let magn = (a * a + b * b).sqrt();
            let freq = ((i as f32) / (BUFFER_SIZE as f32)) * (SAMPLE_RATE as f32);
            (i, freq, magn)
        })
        .collect();

    freqs.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
    let top_list: Vec<&(usize, f32, f32)> = freqs.iter().take(1).collect();

    let (tot, n): (f32, f32) = top_list
    .iter()
    .fold((0.0, 0.0), |(tot, n), (_, freq, magn)| {
        (tot + freq * magn, n + magn)
    }); */
    let transformed = fft(data.to_vec());
    let chart: Vec<f32> = transformed
        .iter()
        .enumerate()
        .map(|(_, &(a, b))| {
            let magn = (a * a + b * b).sqrt();
            magn
        })
        .collect();
    tx.send(FreqEvent { heights: chart })
        .expect("to be able to send");
}

fn error_callback(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
