use std::sync::Mutex;
use crate::{BACKGROUND_COLOR, HEIGHT, WIDTH};

pub struct Buffer {
    pub pixel_vec: Vec<u32>,
    pub depth_vec: Vec<f32>,
}

impl Buffer {
    pub fn new() -> Self {
        let buffer_vec = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];
        Buffer { pixel_vec: buffer_vec, depth_vec: vec![f32::INFINITY; WIDTH * HEIGHT], }
    }

    pub fn buffer_loop(&mut self) {
        self.pixel_vec.iter_mut().for_each(|pixel| {
            *pixel = BACKGROUND_COLOR;
        })
    }

    pub fn update_with_z(buffer: &Mutex<&mut Buffer>, index: usize, value: u32, z: f32) {
        let mut buffer = buffer.lock().unwrap();
        if z < buffer.depth_vec[index] {
            buffer.pixel_vec[index] = value;
            buffer.depth_vec[index] = z;
        }
    }

    pub fn update(buffer: &Mutex<&mut Buffer>, index: usize, value: u32) {
        let mut buffer = buffer.lock().unwrap();
        buffer.pixel_vec[index] = value;
    }

    // pub fn update(&mut self, index: usize, value: u32) {
    //     self.vec[index] = value;
    // }

    pub fn clear(&mut self) {
        self.pixel_vec = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];
        self.depth_vec.fill(f32::INFINITY);
    }

    pub fn get_output(&self) -> &[u32] {
        &self.pixel_vec
    }
}
