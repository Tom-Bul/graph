use std::sync::Mutex;
use crate::{
    buffer::Buffer,
    constants::{HEIGHT, WIDTH, DEPTH},
    DEFAULT_SIZE, LINE_COLOR, POINT_COLOR,
};
use rayon::prelude::*;


pub struct StaticPoint {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
}
pub struct Point {
    pub position: StaticPoint,
    pub size: usize,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    bounce: bool,
}

impl StaticPoint {
    pub fn new() -> Self {
        Self {
            position_x: WIDTH as f32 / 2.0f32,
            position_y: HEIGHT as f32 / 2.0f32,
            position_z: DEPTH as f32 / 2.0f32,
        }
    }

    pub fn get(&self) -> (f32, f32, f32) {
        (self.position_x, self.position_y, self.position_z)
    }
}

impl Point {
    pub fn new(
        position: StaticPoint,
        size: usize,
        velocity_x: f32,
        velocity_y: f32,
        velocity_z: f32,
        bounce: bool,
    ) -> Point {
        Point {
            position,
            size,
            velocity_x,
            velocity_y,
            velocity_z,
            bounce,
        }
    }

    pub fn point_movement(&mut self) {
        let StaticPoint {
            position_x,
            position_y,
            position_z,
        } = &mut self.position;

        *position_x += self.velocity_x * (1.0 / 25.0);
        *position_y += self.velocity_y * (1.0 / 25.0);
        *position_z += self.velocity_z * (1.0 / 25.0);

        if self.bounce {
            self.border_bounce()
        } else {
            self.border_pass()
        }

        self.resize();
    }

    pub fn border_bounce(&mut self) {
        let StaticPoint {
            position_x,
            position_y,
            position_z,
        } = &mut self.position;

        fn get_expression(position: f32, dimension: usize, size: usize) -> bool {
            position >= (dimension - size) as f32 || position <= 0.0
        }

        if get_expression(*position_x, WIDTH, self.size) {
            self.velocity_x = -self.velocity_x;
        }

        if get_expression(*position_y, HEIGHT, self.size) {
            self.velocity_y = -self.velocity_y;
        }

        if get_expression(*position_z, DEPTH, self.size) {
            self.velocity_z = -self.velocity_z;
        }
    }

    pub fn border_pass(&mut self) {
        let StaticPoint {
            position_x,
            position_y,
            ..
        } = &mut self.position;

        if *position_x > WIDTH as f32 {
            *position_x = 0.0;
        } else if *position_x <= 0.0 {
            *position_x = WIDTH as f32;
        };

        if *position_y > HEIGHT as f32 {
            *position_y = 0.0;
        } else if *position_y <= 0.0 {
            *position_y = HEIGHT as f32;
        };
    }

    pub fn draw_point(&self, buffer: &mut Buffer) {
        let StaticPoint {
            position_x,
            position_y,
            position_z,
        } = self.position;

        let buffer = Mutex::new(buffer);

        let range: Vec<(usize, usize)> = (0..self.size)
            .flat_map(|i| (0..self.size).map(move |j| (i, j)))
            .collect();

        range.par_iter().for_each(|&(i, j)| {
            let px = ((position_x - (DEFAULT_SIZE as f32 / 2.0 - 1.0)) as usize + i).min(WIDTH - 1);
            let py = ((position_y - (DEFAULT_SIZE as f32 / 2.0 - 1.0)) as usize + j).min(HEIGHT - 1);
            Buffer::update_with_z(&buffer, py * WIDTH + px, POINT_COLOR, position_z);
        });
    }

    pub fn draw_line(first_pixel: &Point, second_pixel: &Point, buffer: &mut Buffer) {
        let (x1, y1) = (
            first_pixel.position.position_x,
            first_pixel.position.position_y,
        );
        let (x2, y2) = (
            second_pixel.position.position_x,
            second_pixel.position.position_y,
        );

        let slope = (y2 - y1) / (x2 - x1);
        let b = y1 - slope * x1;

        let (start_x, end_x) = if x1 < x2 {
            (x1 as u32, x2 as u32)
        } else {
            (x2 as u32, x1 as u32)
        };

        let buffer = Mutex::new(buffer);

        let range: Vec<u32> = (start_x..=end_x).collect();

        range.par_iter().for_each(|&i| {
            let px = (i as usize).min(WIDTH - 1);
            let py = ((slope * i as f32 + b) as usize).min(HEIGHT - 1);

            Buffer::update(&buffer, py * WIDTH + px, LINE_COLOR);
        });
    }

    fn resize(&mut self) {
        self.size = ((DEPTH as f32 - self.position.position_z) / DEPTH as f32 * 10.0 + 5.0).round() as usize;
    }
}
