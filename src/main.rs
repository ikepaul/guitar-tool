use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, FromSample, Sample, SampleFormat};

fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let data_callback = move |data: &[i32], _: &cpal::InputCallbackInfo| {
        let mut max_sample: i32 = 0;
        for &sample in data {
            max_sample = if max_sample > sample {
                max_sample
            } else {
                sample
            };
        }
        println!("{}", max_sample)
    };

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

    let config = supported_config.into();

    let stream = device
        .build_input_stream(&config, data_callback, error_callback, None)
        .expect("Stream failed to build");

    println!("Starting stream! {:?}", config);
    stream.play().expect("Stream failed to play");

    // Run for 3 seconds before closing.
    println!("Playing for 10 seconds... ");
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);
    println!("Done!");
    Ok(())
}

fn error_callback(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
fn write_silence<T: Sample>(data: &mut [T], _: &cpal::InputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}
