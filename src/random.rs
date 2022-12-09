use rand::prelude::*;

#[inline]
pub fn random_f32_01() -> f32 {
    rand::thread_rng().gen()
}
