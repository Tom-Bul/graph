use std::sync::Mutex;
use crate::{BACKGROUND_COLOR, HEIGHT, WIDTH};

pub struct Buffer {
    pub vec: Vec<u32>,
    pub depth: Vec<f32>,
}

impl Buffer {
    pub fn new() -> Self {
        let buffer_vec = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];
        Buffer { vec: buffer_vec, depth: vec![f32::INFINITY; WIDTH * HEIGHT], }
    }

    pub fn buffer_loop(&mut self) {
        self.vec.iter_mut().for_each(|pixel| {
            *pixel = BACKGROUND_COLOR;
        })
    }

    pub fn update_with_z(buffer: &Mutex<&mut Buffer>, index: usize, value: u32, z: f32) {
        let mut buffer = buffer.lock().unwrap();
        if z < buffer.depth[index] {
            buffer.vec[index] = value;
            buffer.depth[index] = z;
        }
    }

    pub fn update(buffer: &Mutex<&mut Buffer>, index: usize, value: u32) {
        let mut buffer = buffer.lock().unwrap();
        buffer.vec[index] = value;
    }

    // pub fn update(&mut self, index: usize, value: u32) {
    //     self.vec[index] = value;
    // }

    pub fn clear(&mut self) {
        self.vec = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];
        self.depth.fill(f32::INFINITY);
    }

    pub fn get_output(&self) -> &[u32] {
        &self.vec
    }
}
