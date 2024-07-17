use crate::{
    buffer::Buffer,
    constants::{DEPTH, HEIGHT, WIDTH},
    LINE_COLOR, POINT_COLOR,
};

pub struct Point {
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
    pub fn new(
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

    pub fn pixel_movement(&mut self) {
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

    pub fn border_bounce(&mut self) {
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

    pub fn border_pass(&mut self) {
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

    pub fn draw_point(&self, buffer: &mut Buffer) {
        for i in 0..self.size {
            for j in 0..self.size {
                let px = (self.position_x as usize + i).min(WIDTH - 1);
                let py = (self.position_y as usize + j).min(HEIGHT - 1);
                buffer.update(py * WIDTH + px, POINT_COLOR);
            }
        }
    }

    pub fn draw_line(first_pixel: &Point, second_pixel: &Point, buffer: &mut Buffer) {
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
            buffer.update(py * WIDTH + px, LINE_COLOR);
        }
    }

    // fn resize(&mut self) {
    //     self.size = (self.position_z / DEPTH * 90.0 + 10.0).round() as usize;
    // }
}
