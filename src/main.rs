extern crate minifb;
mod buffer;
mod constants;
mod matrix;
mod point;
mod triangle;

use buffer::Buffer;
use constants::*;
use minifb::{Key, Window, WindowOptions};
use point::{Point, StaticPoint};
use triangle::Triangle;

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

    let static_point = StaticPoint::new(WIDTH as f32 / 2.0f32, HEIGHT as f32 / 2.0f32);
    let static_point2 = StaticPoint::new(WIDTH as f32 / 2.0f32, HEIGHT as f32 / 2.0f32);
    let static_point3 = StaticPoint::new(WIDTH as f32 / 2.0f32, HEIGHT as f32 / 2.0f32);

    let mut point = Point::new(
        static_point,
        DEFAULT_SIZE,
        DEFAULT_VELOCITY_X,
        DEFAULT_VELOCITY_Y,
        true,
    );
    let mut point2 = Point::new(
        static_point2,
        DEFAULT_SIZE,
        -DEFAULT_VELOCITY_X - 20.0,
        -DEFAULT_VELOCITY_Y - 10.0,
        true,
    );
    let mut point3 = Point::new(
        static_point3,
        DEFAULT_SIZE,
        -DEFAULT_VELOCITY_X + 10.0,
        DEFAULT_VELOCITY_Y - 10.0,
        true,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.clear();

        point.point_movement();
        point2.point_movement();
        point3.point_movement();

        point.draw_point(&mut buffer);
        point2.draw_point(&mut buffer);
        point3.draw_point(&mut buffer);

        Point::draw_line(&point, &point2, &mut buffer);
        Point::draw_line(&point, &point3, &mut buffer);
        Point::draw_line(&point2, &point3, &mut buffer);

        let triangle = Triangle::new(&point, &point2, &point3);

        triangle.fill(FILL_COLOR, &mut buffer);

        window
            .update_with_buffer(buffer.get_output(), WIDTH, HEIGHT)
            .unwrap();
    }
}
