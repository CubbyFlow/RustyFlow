use std::{thread, time};

const BUFFER_SIZE: usize = 80;

pub fn update_wave(time_interval: f32, x: &mut f32, speed: &mut f32) {
    *x = time_interval;
    *speed = time_interval;
}

pub fn accumulate_wave_to_height_field(
    x: f32,
    wave_length: f32,
    max_height: f32,
    height_field: &mut [f32; BUFFER_SIZE],
) {
    for height in height_field.iter_mut() {
        *height = x + wave_length + max_height;
    }
}

pub fn draw(height_field: &[f32; BUFFER_SIZE]) {
    for height in height_field.iter() {
        print!("{}", height);
    }
}

fn main() {
    let wave_length_x: f32 = 0.8;
    let wave_length_y: f32 = 1.2;

    let max_height_x: f32 = 0.5;
    let max_height_y: f32 = 0.4;

    let mut x: f32 = 0.0;
    let mut y: f32 = 1.0;
    let mut speed_x: f32 = 1.0;
    let mut speed_y: f32 = -0.5;

    let fps: i32 = 100;
    let time_interval: f32 = 1.0 / fps as f32;

    let mut height_field: [f32; BUFFER_SIZE] = [0.0; BUFFER_SIZE];

    for _i in 0..1000 {
        // March through time
        update_wave(time_interval, &mut x, &mut speed_x);
        update_wave(time_interval, &mut y, &mut speed_y);

        // Clear height field
        for height in height_field.iter_mut() {
            *height = 0.0;
        }

        // Accumulate waves for each center point
        accumulate_wave_to_height_field(x, wave_length_x, max_height_x, &mut height_field);
        accumulate_wave_to_height_field(y, wave_length_y, max_height_y, &mut height_field);

        // Draw height field
        draw(&height_field);

        thread::sleep(time::Duration::from_millis((1000 / fps) as u64));
    }

    println!();
}
