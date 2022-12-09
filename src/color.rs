use std::ops::{Add, Div, Mul};

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

    #[inline]
    pub fn zero() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, other: f32) -> Color {
        Color::new(self.r * other, self.g * other, self.b * other)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    #[inline]
    fn mul(self, other: Color) -> Color {
        Color::new(self * other.r, self * other.g, self * other.b)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Div<f32> for Color {
    type Output = Color;

    #[inline]
    fn div(self, other: f32) -> Color {
        Color::new(self.r / other, self.g / other, self.b / other)
    }
}

pub fn clampf32(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn image_to_u8(image: &[Color]) -> Vec<u8> {
    image
        .iter()
        .fold(Vec::with_capacity(image.len() * 3), |mut acc, c| {
            // Gamma correct the color with gamma=2.0
            let r = (clampf32(c.r.sqrt(), 0.0, 0.999) * 256.0) as u8;
            let g = (clampf32(c.g.sqrt(), 0.0, 0.999) * 256.0) as u8;
            let b = (clampf32(c.b.sqrt(), 0.0, 0.999) * 256.0) as u8;
            acc.push(r);
            acc.push(g);
            acc.push(b);
            acc
        })
}
