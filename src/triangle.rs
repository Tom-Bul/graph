use std::sync::Mutex;
use crate::{buffer::Buffer, matrix::Matrix, point::Point, WIDTH};
use rayon::prelude::*;
use crate::point::StaticPoint;

pub struct Triangle<'a> {
    p1: &'a Point,
    p2: &'a Point,
    p3: &'a Point,
}

impl<'a> Triangle<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point, p3: &'a Point) -> Self {
        Triangle { p1, p2, p3 }
    }

    pub fn fill(&self, buffer: &mut Buffer, color: u32) {
        let (x1, y1, x2, y2, x3, y3) = (self.p1.position.position_x, self.p1.position.position_y, self.p2.position.position_x, self.p2.position.position_y, self.p3.position.position_x, self.p3.position.position_y);
        let matrix_abc = Matrix::new([
            [ x1, y1, 1.0, ], [ x2, y2, 1.0, ], [ x3, y3, 1.0, ],
        ])
        .det()
        .unwrap_or(0.0);

        let position_y_collection: Vec<usize> = vec![
            y1 as usize,
            y2 as usize,
            y3 as usize,
        ];
        let position_x_collection: Vec<usize> = vec![
            x1 as usize,
            x2 as usize,
            x3 as usize,
        ];

        let (upper_bound_y, lower_bound_y, upper_bound_x, lower_bound_x) = (
            position_y_collection.iter().max().unwrap(),
            position_y_collection.iter().min().unwrap(),
            position_x_collection.iter().max().unwrap(),
            position_x_collection.iter().min().unwrap(),
        );

        let buffer = Mutex::new(buffer);

        let indices_to_process: Vec<usize> = (*lower_bound_y..=*upper_bound_y)
            .flat_map(|y| (*lower_bound_x..=*upper_bound_x).map(move |x| y * WIDTH + x))
            .collect();

        indices_to_process.par_iter().for_each(|&index| {
            let (x, y) = (index % WIDTH, index / WIDTH);

            let matrix_pbc = match Matrix::new([
                [x as f32, y as f32, 1.0], [ x2, y2, 1.0, ], [ x3, y3, 1.0, ],
            ])
                .det()
            {
                Some(determinant) if determinant != 0.0 => determinant,
                _ => return,
            };
            let matrix_apc = match Matrix::new([
                [ x1, y1, 1.0, ], [x as f32, y as f32, 1.0], [ x3, y3, 1.0, ],
            ])
                .det()
            {
                Some(determinant) if determinant != 0.0 => determinant,
                _ => return,
            };
            let matrix_abp = match Matrix::new([
                [ x1, y1, 1.0, ], [ x2, y2, 1.0, ], [x as f32, y as f32, 1.0],
            ])
                .det()
            {
                Some(determinant) if determinant != 0.0 => determinant,
                _ => return,
            };

            let z = find_z(&self.p1.position, &self.p2.position, &self.p3.position, (x,y));

            let (a, b, c) = (
                matrix_pbc / matrix_abc,
                matrix_apc / matrix_abc,
                matrix_abp / matrix_apc,
            );

            if a >= 0.0 && b >= 0.0 && c >= 0.0 {

                Buffer::update_with_z(&buffer, index, get_color(z,color), z as f32);
            }
        });
    }
}

pub fn find_z(p1: &StaticPoint, p2: &StaticPoint, p3: &StaticPoint, p: (usize, usize)) -> usize {
    let (x1, y1, z1, x2, y2, z2, x3, y3, z3) =
        (p1.position_x, p1.position_y, p1.position_z,
         p2.position_x, p2.position_y, p2.position_z,
         p3.position_x, p3.position_y, p3.position_z);

    let lam_1 = ((y2 - y3) * (p.0 as f32 - x3) + (x3 - x2) * (p.1 as f32 - y3)) /
        ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
    let lam_2 = ((y3 - y1) * (p.0 as f32 - x3) + (x1 - x3) * (p.1 as f32 - y3)) /
        ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
    let lam_3 = 1.0 - lam_1 - lam_2;

    ( lam_1 * z1 + lam_2 * z2 + lam_3 * z3 ) as usize
}

pub fn get_color(z: usize, color: u32) -> u32 {
    let red = ((color >> 16) & 0xFF) as f32;
    let green = ((color >> 8) & 0xFF) as f32;
    let blue = (color & 0xFF) as f32;

    let factor = 1.0 - (z as f32 / 300.0);
    let darkened_red = (red * factor).min(255.0).max(0.0) as u32;
    let darkened_green = (green * factor).min(255.0).max(0.0) as u32;
    let darkened_blue = (blue * factor).min(255.0).max(0.0) as u32;

    (darkened_red << 16) | (darkened_green << 8) | darkened_blue
}
