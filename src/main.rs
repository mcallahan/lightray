mod camera;
mod color;
mod hittable;
mod random;
mod ray;
mod vector;

use camera::Camera;
use color::{image_to_u8, Color};
use hittable::{Hittable, HittableList, Sphere};
use random::{random_f32_01, random_in_hemisphere, random_unit_vector, random_in_unit_sphere};
use ray::Ray;
use vector::Point3;

use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::sync::Arc;

fn ray_color(r: &Ray, world: &Arc<dyn Hittable + Sync + Send>, depth: i32) -> Color {
    if depth <= 0 {
	Color::zero()
    } else if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        //let target = rec.point + rec.normal + random_in_unit_sphere();
	//let target = rec.point + rec.normal + random_unit_vector();
	let target = rec.point + random_in_hemisphere(rec.normal);
	let newray = Ray::new(rec.point, target - rec.point);
        0.5 * ray_color(&newray, world, depth-1)
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    let image_width = 400;
    let image_height = 225;
    let aspect_ratio = image_width as f32 / image_height as f32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world: Arc<dyn Hittable + Sync + Send> = Arc::new(world);

    // Camera
    let camera = Camera::new(aspect_ratio);

    // Used to convert from [0.0,1.0] to viewport(image) space.
    let rwidth = 1.0 / (image_width - 1) as f32;
    let rheight = 1.0 / (image_height - 1) as f32;

    // Per pixel parallel render.
    let image: Vec<Color> = (0..image_width * image_height)
        .into_par_iter()
        .map(|pidx: usize| {
            // Compute image coordinates in camera space.
            let i = (pidx % image_width) as f32;
            let j = (image_height - 1 - (pidx / image_width)) as f32;
            (0..samples_per_pixel).fold(Color::zero(), |acc, _| {
                let u = (i + random_f32_01()) * rwidth;
                let v = (j + random_f32_01()) * rheight;
                let r = camera.get_ray(u, v);
                acc + ray_color(&r, &world, max_depth)
            }) / samples_per_pixel as f32
        })
        .collect();

    // Write a binary RGB PNM to stdout
    println!("P6");
    println!("{image_width} {image_height}");
    println!("255");
    io::stdout().write_all(&image_to_u8(&image)).unwrap();
}
