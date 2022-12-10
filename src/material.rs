use crate::color::{clampf32, Color};
use crate::hittable::HitRec;
use crate::random::{random_f32_01, random_in_hemisphere, random_in_unit_sphere};
use crate::ray::Ray;
use crate::vector::{dot, Vector3};

#[inline]
fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * dot(v, n) * n
}

#[inline]
fn refract(uv: Vector3, n: Vector3, etai_over_etat: f32) -> Vector3 {
    let cos_theta = f32::min(dot(-uv, n), 1.0);
    let perpendicular = etai_over_etat * (uv + cos_theta * n);
    let parallel = -(1.0 - perpendicular.length_squared()).abs().sqrt() * n;
    perpendicular + parallel
}

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

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }

    #[inline]
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRec) -> Option<(Color, Ray)> {
        let ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let udir = r.direction.unit_vector();
        let cos_theta = f32::min(dot(-udir, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ratio) > random_f32_01() {
                reflect(udir, rec.normal)
            } else {
                refract(udir, rec.normal, ratio)
            };
        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.point, direction)))
    }
}
