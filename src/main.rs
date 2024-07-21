extern crate minifb;
mod buffer;
mod constants;
mod matrix;
mod point;
mod triangle;
mod random;

use buffer::Buffer;
use constants::*;
use minifb::{Key, Window, WindowOptions};
use point::Point;
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

    let mut points = Point::new_multiple(POINTS_NUMBER);
    let points_len = points.len();
    let indices: Vec<usize> = (0..points.len()).collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.clear();

        indices.iter().for_each(|&index| {
            let point = &mut points[index];
            point.point_movement();
            // point.draw_point(&mut buffer);

            // for &i in &indices[index + 1..] {
            //     Point::draw_line(&points[index], &points[i], &mut buffer);
            // }
        });

        let mut color_index = 0;

        for i in 0..points_len {
            for j in (i + 1)..points_len {
                for k in (j + 1)..points_len {
                    let triangle = Triangle::new(&points[i], &points[j], &points[k]);
                    triangle.fill(&mut buffer, FILL_COLORS[color_index]);
                    color_index += 1;
                }
            }
        }

        window
            .update_with_buffer(buffer.get_output(), WIDTH, HEIGHT)
            .unwrap();
    }
}
