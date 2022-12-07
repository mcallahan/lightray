mod color;
mod vector;

use color::{image_to_u8, Color};

use rayon::prelude::*;
use std::io;
use std::io::Write;

fn main() {
    let image_width = 256;
    let image_height = 256;

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

            // Simple gradient based upon pixel location.
            let r = i * rwidth;
            let g = j * rheight;
            let b = 0.25;
            Color::new(r, g, b)
        })
        .collect();

    // Write a binary RGB PNM to stdout
    println!("P6");
    println!("{image_width} {image_height}");
    println!("255");
    io::stdout().write_all(&image_to_u8(&image)).unwrap();
}
