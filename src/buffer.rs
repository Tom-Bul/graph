use crate::{BACKGROUND_COLOR, HEIGHT, WIDTH};

pub struct Buffer {
    pub vec: Vec<u32>,
}

impl Buffer {
    pub fn new() -> Self {
        let buffer_vec = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];
        Buffer { vec: buffer_vec }
    }

    pub fn buffer_loop(&mut self) {
        self.vec.iter_mut().for_each(|pixel| {
            *pixel = BACKGROUND_COLOR;
        })
    }

    pub fn update(&mut self, index: usize, value: u32) {
        self.vec[index] = value;
    }

    pub fn clear(&mut self) {
        self.vec = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];
    }

    pub fn get_output(&mut self) -> &[u32] {
        &self.vec[..]
    }
}
