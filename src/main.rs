mod color;
mod hittable;
mod ray;
mod vector;

use color::{image_to_u8, Color};
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vector::{dot, Point3, Vector3};

use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::sync::Arc;

fn ray_color(r: &Ray, world: &Arc<dyn Hittable + Sync + Send>) -> Color {
    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        let normal = rec.normal;
        0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
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

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world: Arc<dyn Hittable + Sync + Send> = Arc::new(world);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

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

            let u = i * rwidth;
            let v = j * rheight;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            ray_color(&r, &world)
        })
        .collect();

    // Write a binary RGB PNM to stdout
    println!("P6");
    println!("{image_width} {image_height}");
    println!("255");
    io::stdout().write_all(&image_to_u8(&image)).unwrap();
}
