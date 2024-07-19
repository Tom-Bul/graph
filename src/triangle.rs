use std::sync::Mutex;
use crate::{buffer::Buffer, matrix::Matrix, point::Point, WIDTH};
use rayon::prelude::*;

pub struct Triangle<'a> {
    p1: &'a Point,
    p2: &'a Point,
    p3: &'a Point,
}

impl<'a> Triangle<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point, p3: &'a Point) -> Self {
        Triangle { p1, p2, p3 }
    }

    pub fn fill(&self, color: u32, buffer: &mut Buffer) {
        let matrix_abc = Matrix::new([
            [
                self.p1.position.position_x,
                self.p1.position.position_y,
                1.0,
            ],
            [
                self.p2.position.position_x,
                self.p2.position.position_y,
                1.0,
            ],
            [
                self.p3.position.position_x,
                self.p3.position.position_y,
                1.0,
            ],
        ])
        .det()
        .unwrap_or(0.0);

        let position_y_collection: Vec<usize> = vec![
            self.p1.position.position_y as usize,
            self.p2.position.position_y as usize,
            self.p3.position.position_y as usize,
        ];
        let position_x_collection: Vec<usize> = vec![
            self.p1.position.position_x as usize,
            self.p2.position.position_x as usize,
            self.p3.position.position_x as usize,
        ];

        let (upper_bound_y, lower_bound_y, upper_bound_x, lower_bound_x) = (
            position_y_collection.iter().max().unwrap(),
            position_y_collection.iter().min().unwrap(),
            position_x_collection.iter().max().unwrap(),
            position_x_collection.iter().min().unwrap(),
        );

        let buffer = Mutex::new(buffer);

        // Create a range of indices to process
        let indices_to_process: Vec<usize> = (*lower_bound_y..=*upper_bound_y)
            .flat_map(|y| (*lower_bound_x..=*upper_bound_x).map(move |x| y * WIDTH + x))
            .collect();

        // Use par_iter from Rayon to parallelize the loop
        indices_to_process.par_iter().for_each(|&index| {
            let (x, y) = (index % WIDTH, index / WIDTH);

            let matrix_pbc = match Matrix::new([
                [x as f32, y as f32, 1.0],
                [
                    self.p2.position.position_x,
                    self.p2.position.position_y,
                    1.0,
                ],
                [
                    self.p3.position.position_x,
                    self.p3.position.position_y,
                    1.0,
                ],
            ])
                .det()
            {
                Some(determinant) if determinant != 0.0 => determinant,
                _ => return,
            };
            let matrix_apc = match Matrix::new([
                [
                    self.p1.position.position_x,
                    self.p1.position.position_y,
                    1.0,
                ],
                [x as f32, y as f32, 1.0],
                [
                    self.p3.position.position_x,
                    self.p3.position.position_y,
                    1.0,
                ],
            ])
                .det()
            {
                Some(determinant) if determinant != 0.0 => determinant,
                _ => return,
            };
            let matrix_abp = match Matrix::new([
                [
                    self.p1.position.position_x,
                    self.p1.position.position_y,
                    1.0,
                ],
                [
                    self.p2.position.position_x,
                    self.p2.position.position_y,
                    1.0,
                ],
                [x as f32, y as f32, 1.0],
            ])
                .det()
            {
                Some(determinant) if determinant != 0.0 => determinant,
                _ => return,
            };

            let (a, b, c) = (
                matrix_pbc / matrix_abc,
                matrix_apc / matrix_abc,
                matrix_abp / matrix_apc,
            );

            if a >= 0.0 && b >= 0.0 && c >= 0.0 && a + b + c != 1.0 {

                Buffer::update(&buffer, index, color);
            }
        });
    }
}
