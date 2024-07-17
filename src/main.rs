extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const DEPTH: f32 = 400.0;

const DEFAULT_VELOCITY_X: f32 = 40.0;
const DEFAULT_VELOCITY_Y: f32 = 39.0;
const DEFAULT_VELOCITY_Z: f32 = 40.0;

struct Point {
    position_x: f32,
    position_y: f32,
    position_z: f32,
    size: usize,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    bounce: bool,
}

impl Point {
    fn new(
        position_x: f32,
        position_y: f32,
        position_z: f32,
        size: usize,
        velocity_x: f32,
        velocity_y: f32,
        velocity_z: f32,
        bounce: bool,
    ) -> Point {
        Point {
            position_x,
            position_y,
            position_z,
            size,
            velocity_x,
            velocity_y,
            velocity_z,
            bounce,
        }
    }

    fn pixel_movement(&mut self) {
        self.position_x += self.velocity_x * (1.0 / 60.0);
        self.position_y += self.velocity_y * (1.0 / 60.0);
        self.position_z += self.velocity_z * (1.0 / 60.0);

        if self.bounce {
            self.border_bounce()
        } else {
            self.border_pass()
        }

        // self.resize();
    }

    fn border_bounce(&mut self) {
        fn get_expression_x(position_x: f32, size: usize) -> bool {
            position_x >= (WIDTH - size) as f32 || position_x <= 0.0
        }

        fn get_expression_y(position_y: f32, size: usize) -> bool {
            position_y >= (HEIGHT - size) as f32 || position_y <= 0.0
        }

        if get_expression_x(self.position_x, self.size) {
            self.velocity_x = -self.velocity_x;
        }

        if get_expression_y(self.position_y, self.size) {
            self.velocity_y = -self.velocity_y;
        }

        if self.position_z > DEPTH || self.position_z <= 0.0 {
            self.velocity_z = -self.velocity_z;
        }
    }

    fn border_pass(&mut self) {
        if self.position_x > WIDTH as f32 {
            self.position_x = 0.0;
        } else if self.position_x <= 0.0 {
            self.position_x = WIDTH as f32;
        };

        if self.position_y > HEIGHT as f32 {
            self.position_y = 0.0;
        } else if self.position_y <= 0.0 {
            self.position_y = HEIGHT as f32;
        };
    }

    fn draw_point(&self, buffer: &mut Vec<u32>) {
        for i in 0..self.size {
            for j in 0..self.size {
                let px = (self.position_x as usize + i).min(WIDTH - 1);
                let py = (self.position_y as usize + j).min(HEIGHT - 1);
                buffer[py * WIDTH + px] = 0x00FF00;
            }
        }
    }

    fn draw_line(first_pixel: &Point, second_pixel: &Point, buffer: &mut Vec<u32>) {
        let (x1, y1) = (
            first_pixel.position_x + (first_pixel.size / 2) as f32,
            first_pixel.position_y + (first_pixel.size / 2) as f32,
        );
        let (x2, y2) = (
            second_pixel.position_x + (second_pixel.size / 2) as f32,
            second_pixel.position_y + (second_pixel.size / 2) as f32,
        );

        let slope = (y2 - y1) / (x2 - x1);
        let b = y1 - slope * x1;

        let (start_x, end_x) = if x1 < x2 {
            (x1 as u32, x2 as u32)
        } else {
            (x2 as u32, x1 as u32)
        };

        for i in start_x..=end_x {
            let px = (i as usize).min(WIDTH - 1);
            let py = ((slope * i as f32 + b) as usize).min(HEIGHT - 1);
            buffer[py * WIDTH + px] = 0x00FF00;
        }
    }

    // fn resize(&mut self) {
    //     self.size = (self.position_z / DEPTH * 90.0 + 10.0).round() as usize;
    // }
}

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

        // pixel.save_prev_instance();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
