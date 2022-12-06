#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

fn clampf32(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

// Convert a Color image into u8 image (RGB ordered).
pub fn image_to_u8(image: &[Color]) -> Vec<u8> {
    image
        .iter()
        .fold(Vec::with_capacity(image.len() * 3), |mut acc, c| {
            let r = (clampf32(c.r, 0.0, 0.999) * 256.0) as u8;
            let g = (clampf32(c.g, 0.0, 0.999) * 256.0) as u8;
            let b = (clampf32(c.b, 0.0, 0.999) * 256.0) as u8;
            acc.push(r);
            acc.push(g);
            acc.push(b);
            acc
        })
}
