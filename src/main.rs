extern crate minifb;
mod constants;
mod point;

use constants::*;
use minifb::{Key, Window, WindowOptions};
use point::Point;

fn main() {
    let mut window = Window::new(
        "graph 0.0.1",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut point = Point::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0,
        DEPTH as f32 / 2.0,
        10,
        DEFAULT_VELOCITY_X,
        DEFAULT_VELOCITY_Y,
        DEFAULT_VELOCITY_Z,
        true,
    );
    let mut point2 = Point::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0,
        DEPTH as f32 / 2.0,
        10,
        -DEFAULT_VELOCITY_X - 20.0,
        -DEFAULT_VELOCITY_Y - 10.0,
        DEFAULT_VELOCITY_Z,
        true,
    );
    let mut point3 = Point::new(
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0,
        DEPTH as f32 / 2.0,
        10,
        -DEFAULT_VELOCITY_X + 10.0,
        DEFAULT_VELOCITY_Y - 10.0,
        DEFAULT_VELOCITY_Z,
        true,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for pixel in buffer.iter_mut() {
            *pixel = 0;
        }

        point.pixel_movement();
        point2.pixel_movement();
        point3.pixel_movement();

        point.draw_point(&mut buffer);
        point2.draw_point(&mut buffer);
        point3.draw_point(&mut buffer);

        Point::draw_line(&point, &point2, &mut buffer);
        Point::draw_line(&point, &point3, &mut buffer);
        Point::draw_line(&point2, &point3, &mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
