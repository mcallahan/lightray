use crate::color::Color;
use crate::vector::{dot, Vector3};
use rand::prelude::*;

#[inline]
pub fn random_f32_01() -> f32 {
    rand::thread_rng().gen()
}

#[inline]
pub fn random_f32(min: f32, max: f32) -> f32 {
    // Returns a f32 uniformly distributed between [min, max)
    min + (max - min) * random_f32_01()
}

impl Vector3 {
    #[inline]
    pub fn random(min: f32, max: f32) -> Self {
        Self::new(
            random_f32(min, max),
            random_f32(min, max),
            random_f32(min, max),
        )
    }
}

impl Color {
    #[inline]
    pub fn random(min: f32, max: f32) -> Self {
        Self::new(
            random_f32(min, max),
            random_f32(min, max),
            random_f32(min, max),
        )
    }
}

#[inline]
pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(random_f32(-1.0, 0.0), random_f32(-1.0, 0.0), 0.0);
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vector3 {
    random_in_unit_sphere().unit_vector()
}

#[inline]
pub fn random_in_hemisphere(normal: Vector3) -> Vector3 {
    let sphere = random_in_unit_sphere();
    if dot(sphere, normal) > 0.0 {
        sphere // same hemisphere
    } else {
        -sphere // other hemisphere, flip
    }
}
