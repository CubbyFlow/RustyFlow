use std::{thread, time};

const BUFFER_SIZE: usize = 80;
const GRAY_SCALE_TABLE_SIZE: usize = 10;
const GRAY_SCALE_TABLE: [char; GRAY_SCALE_TABLE_SIZE] =
    [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

pub fn update_wave(time_interval: f32, x: &mut f32, speed: &mut f32) {
    *x += time_interval * (*speed);

    // Boundary reflection
    if *x > 1.0 {
        *speed *= -1.0;
        *x = 1.0 + time_interval * (*speed);
    } else if *x < 0.0 {
        *speed *= -1.0;
        *x = time_interval * (*speed);
    }
}

pub fn accumulate_wave_to_height_field(
    x: f32,
    wave_length: f32,
    max_height: f32,
    height_field: &mut [f32; BUFFER_SIZE],
) {
    let quarter_wave_length: f32 = 0.25 * wave_length;
    let start: i32 = ((x - quarter_wave_length) * (BUFFER_SIZE as f32)) as i32;
    let end: i32 = ((x + quarter_wave_length) * (BUFFER_SIZE as f32)) as i32;

    for i in start..end {
        let mut i_new: i32 = i;

        if i < 0 {
            i_new = -i - 1;
        } else if i >= BUFFER_SIZE as i32 {
            i_new = 2 * BUFFER_SIZE as i32 - i - 1;
        }

        let distance: f32 = ((i as f32 + 0.5) / BUFFER_SIZE as f32 - x).abs();
        let height: f32 = max_height
            * 0.5
            * ((distance * std::f32::consts::PI / quarter_wave_length)
                .min(std::f32::consts::PI)
                .cos()
                + 1.0);

        height_field[i_new as usize] += height;
    }
}

pub fn draw(height_field: &[f32; BUFFER_SIZE]) {
    let mut buffer: [char; BUFFER_SIZE] = [' '; BUFFER_SIZE];

    // Convert height field to grayscale
    for i in 0..BUFFER_SIZE {
        let height: f32 = height_field[i];
        let table_index: usize = ((GRAY_SCALE_TABLE_SIZE as f32 * height).floor() as usize)
            .min(GRAY_SCALE_TABLE_SIZE - 1);
        buffer[i] = GRAY_SCALE_TABLE[table_index];
    }

    // Clear old prints
    print!("\r");

    for val in buffer.iter() {
        print!("{}", val);
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
