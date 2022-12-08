use crate::ray::Ray;
use crate::vector::{dot, Point3, Vector3};
use std::sync::Arc;

pub struct HitRec {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRec {
    #[inline]
    fn get_face_normal(r: &Ray, outward_normal: Vector3) -> (bool, Vector3) {
        let front_face = dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRec>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    #[inline]
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRec> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b - sqrtd) / a;
            if root < tmin || tmax < root {
                return None;
            }
        }

        let t = root;
        let point = r.at(t);
        let outie = (point - self.center) / self.radius;
        let (front_face, normal) = HitRec::get_face_normal(r, outie);
        Some(HitRec {
            point,
            normal,
            t,
            front_face,
        })
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    #[inline]
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    #[inline]
    pub fn add(&mut self, obj: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRec> {
        let mut rec = None;
        let mut closest_so_far = tmax;

        for object in &self.objects {
            if let Some(tmprec) = object.hit(r, tmin, closest_so_far) {
                closest_so_far = tmprec.t;
                rec = Some(tmprec);
            }
        }
        rec
    }
}
