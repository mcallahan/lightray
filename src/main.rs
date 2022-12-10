mod camera;
mod color;
mod hittable;
mod material;
mod random;
mod ray;
mod vector;

use camera::Camera;
use color::{image_to_u8, Color};
use hittable::{Hittable, HittableList, Sphere};
use material::{Dielectric, Lambertian, Metal};
use random::random_f32_01;
use ray::Ray;
use vector::{Point3, Vector3};

use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::sync::Arc;

fn ray_color(r: &Ray, world: &Arc<dyn Hittable + Sync + Send>, depth: i32) -> Color {
    if depth <= 0 {
        Color::zero()
    } else if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::zero()
        }
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
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
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let world: Arc<dyn Hittable + Sync + Send> = Arc::new(world);

    // Camera
    let lookfrom = Point3::new(-2.0, 2.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    let camera = Camera::new(lookfrom, lookat, vup, fov, aspect_ratio);

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
