use crate::color::{clampf32, Color};
use crate::hittable::HitRec;
use crate::random::{random_in_hemisphere, random_in_unit_sphere};
use crate::ray::Ray;
use crate::vector::{dot, reflect}; // circular dep.

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRec) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRec) -> Option<(Color, Ray)> {
        let mut scatter_direction = random_in_hemisphere(rec.normal);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = clampf32(fuzz, 0.0, 1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRec) -> Option<(Color, Ray)> {
        let reflected = reflect(r.direction.unit_vector(), rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere());
        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
