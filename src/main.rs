extern crate minifb;
mod buffer;
mod constants;
mod matrix;
mod point;

use buffer::Buffer;
use constants::*;
use matrix::Matrix;
use minifb::{Key, Window, WindowOptions};
use point::Point;

fn main() {
    let mut window = Window::new(
        "graph",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut buffer = Buffer::new();

    buffer.buffer_loop();

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

    let matrix = Matrix::new([[1, 2, 1], [0, 3, 4], [3, 1, 4]]);

    println!("matrix {:?}", matrix.det().unwrap_or(0));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.clear();

        point.pixel_movement();
        point2.pixel_movement();
        point3.pixel_movement();

        point.draw_point(&mut buffer);
        point2.draw_point(&mut buffer);
        point3.draw_point(&mut buffer);

        Point::draw_line(&point, &point2, &mut buffer);
        Point::draw_line(&point, &point3, &mut buffer);
        Point::draw_line(&point2, &point3, &mut buffer);

        window
            .update_with_buffer(buffer.get_output(), WIDTH, HEIGHT)
            .unwrap();
    }
}
