use std::f64::consts::PI;

use bevy::app::{App, Startup};
use bevy::ecs::system::Commands;
use bevy::sprite::ColorMaterial;
use bevy::DefaultPlugins;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Data, FromSample, Sample, SampleFormat, SampleRate, StreamConfig};

const SAMPLE_RATE: u32 = 384000;
const BUFFER_SIZE: u32 = (2 as u32).pow(14);

fn main() -> Result<(), ()> {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_bars)
        .run();

    /*  let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    let output = fft(input);

    println!("{:?}", output); */

    // listen_to_mic();
    Ok(())
}

fn update_bars(mut query: Query<&Bar>) {
    for bar in &query {
        println!("{}", bar.height);
    }
}

#[derive(Component)]
struct Bar {
    freq: f64,
    height: f64,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    /* //Circle
       commands.spawn(MaterialMesh2dBundle {
           mesh: meshes.add(shape::Circle::new(50.).into()).into(),
           material: materials.add(ColorMaterial::from(Color::PURPLE)),
           transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),

           ..default()
       });
    */
    // Rectangle
    commands.spawn((
        Bar {
            freq: 1.0,
            height: 0.0,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 0.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
    ));
    // Quad
    /* commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    }); */
}

fn listen_to_mic() -> Result<(), ()> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("no input device available");

    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");

    let mut supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let mut config: StreamConfig = supported_config.into();
    config.buffer_size = BufferSize::Fixed(BUFFER_SIZE);
    config.sample_rate = SampleRate(SAMPLE_RATE);

    let stream = device
        .build_input_stream(&config, data_callback, error_callback, None)
        .expect("Stream failed to build");

    println!("Starting stream! {:?}", config);
    stream.play().expect("Stream failed to play");

    // Run for 3 seconds before closing.
    std::thread::sleep(std::time::Duration::from_nanos(
        (BUFFER_SIZE / SAMPLE_RATE * 1_000_000_000) as u64,
    ));
    drop(stream);
    println!("Done!");
    Ok(())
}

fn fft(samples: Vec<f64>) -> Vec<(f64, f64)> {
    let size = samples.len();
    if size == 1 {
        return vec![(samples[0], 0.0)];
    };

    let (even_samples, odd_samples) = evens_and_odds(samples);

    let even_res = fft(even_samples);
    let odd_res = fft(odd_samples);

    let mut out: Vec<(f64, f64)> = vec![(0.0, 0.0); size];

    for k in 0..(size / 2) {
        let (o_real, o_imaginary) = odd_res[k];
        let (e_real, e_imaginary) = even_res[k];

        let angle: f64 = -2.0 * PI * (k as f64) / (size as f64);

        let t_real = angle.cos() * o_real - angle.sin() * o_imaginary;
        let t_imaginary = angle.sin() * o_real + angle.cos() * o_imaginary;

        out[k] = (e_real + t_real, e_imaginary + t_imaginary);
        out[k + size / 2] = (e_real - t_real, e_imaginary - t_imaginary);
    }
    out
}

fn evens_and_odds<T: Copy>(arr: Vec<T>) -> (Vec<T>, Vec<T>) {
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for (index, &element) in arr.iter().enumerate() {
        if index % 2 == 0 {
            evens.push(element);
        } else {
            odds.push(element);
        }
    }
    (evens, odds)
}

fn data_callback(data: &[f64], _: &cpal::InputCallbackInfo) {
    let mut max_sample: f64 = 0.0;
    for &sample in data {
        max_sample = if max_sample > sample {
            max_sample
        } else {
            sample
        };
    }
    /* if (max_sample > 1.0) {
        let transformed = fft(data.to_vec());
        let freqs: Vec<f64> = transformed
            .iter()
            .enumerate()
            .take(200)
            .filter_map(|(i, &(a, b))| {
                let magn = (a * a + b * b).sqrt();
                if magn > 1000.0 {
                    Some(((i as f64) / 16384.0) * 384000.0)
                } else {
                    None
                }
            })
            .collect();

        println!("{:?}", freqs)
    } */
    if max_sample > 0.05 {
        let transformed = fft(data.to_vec());
        let mut freqs: Vec<(usize, f64, f64)> = transformed
            .iter()
            .enumerate()
            .take(100)
            .map(|(i, &(a, b))| {
                let magn = (a * a + b * b).sqrt();
                let freq = ((i as f64) / (BUFFER_SIZE as f64)) * (SAMPLE_RATE as f64);
                (i, freq, magn)
            })
            .collect();
        freqs.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
        let top_list: Vec<&(usize, f64, f64)> = freqs.iter().take(1).collect();

        let (tot, n): (f64, f64) = top_list
            .iter()
            .fold((0.0, 0.0), |(tot, n), (_, freq, magn)| {
                (tot + freq * magn, n + magn)
            });

        println!("{:?}", top_list);
    }
}

fn error_callback(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
fn write_silence<T: Sample>(data: &mut [T], _: &cpal::InputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}
