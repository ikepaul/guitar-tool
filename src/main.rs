pub mod constants;
pub mod eventhelper;
pub mod fft;
pub mod mic;

use crate::constants::{NUM_OF_BARS, SCREEN_WIDTH};
use crate::eventhelper::*;
use crate::mic::{mic_setup, FreqEvent};

use std::{
    sync::{self},
    thread,
};

use bevy::{ecs::system::Commands, prelude::*};

fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let (tx, rx) = sync::mpsc::channel::<FreqEvent>();

    thread::spawn(move || mic_setup(tx));

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_event_channel(rx)
        .add_systems(Update, update_bars)
        .add_systems(Update, update_bars_resource)
        .add_systems(Update, redraw_bars)
        .add_event::<FreqEvent>()
        .run();

    /*  let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    let output = fft(input);println!("{:?}", bars.heights[bar.i])

    println!("{:?}", output); */

    Ok(())
}

fn update_bars_resource(mut res: ResMut<Bars>, mut freq_event: EventReader<FreqEvent>) {
    let bars_o = freq_event.read().last();
    match bars_o {
        Some(bars) => {
            res.bars = bars
                .heights
                .iter()
                .enumerate()
                .map(|(i, h)| Bar { i, height: *h })
                .collect();
        }
        None => (),
    }
}

//Dont know if i should update Bars resource when event happens or if i should directly update the individual bars
/* fn update_bars(mut query: Query<&mut Bar>, mut freq_event: EventReader<FreqEvent>) {
    let bars_o = freq_event.read().last();
    match bars_o {
        Some(bars) => {
            for mut bar in &mut query {
                bar.height = bars.heights[bar.i];
            }
        }
        None => (),
    }
} */
fn update_bars(mut query: Query<&mut Bar>, res: Res<Bars>) {
    for mut bar in &mut query {
        if bar.i < res.bars.len() {
            bar.height = res.bars[bar.i].height;
        }
    }
}
fn redraw_bars(mut query: Query<(&Bar, &mut Sprite, &mut Transform)>) {
    for (bar, mut sprite, mut transform) in query.iter_mut() {
        sprite.custom_size = Some(Vec2::new((SCREEN_WIDTH / NUM_OF_BARS) as f32, bar.height));
        // transform.translation = Some(Vec2::new((SCREEN_WIDTH / NUM_OF_BARS) as f32, bar.height));
    }
}

#[derive(Component, Copy, Clone)]
struct Bar {
    i: usize,
    height: f32,
}

#[derive(Resource)]
struct Bars {
    bars: Vec<Bar>,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut bars = Vec::new();

    for i in 2..(NUM_OF_BARS as usize) {
        // Rectangle
        let bar = Bar { i, height: 0.0 };

        commands.spawn((
            bar,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new((SCREEN_WIDTH / NUM_OF_BARS) as f32, 0.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (SCREEN_WIDTH / NUM_OF_BARS) as f32 * (i as f32) - (SCREEN_WIDTH / 2) as f32,
                    0.,
                    0.,
                )),
                ..default()
            },
        ));

        bars.push(bar);
    }

    commands.insert_resource(Bars { bars })
}
