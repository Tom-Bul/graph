use rand::prelude::*;

pub fn get_rand_velocity(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    let rand_number: f32 = rng.gen();
    (rand_number - 0.5) * 2.0 * (max - min) + min
}
