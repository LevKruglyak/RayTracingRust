use image::Rgb;
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Color(Rgb<f32>);

impl Color {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            0: image::Rgb { 0: [r, g, b] },
        }
    }

    #[inline]
    pub fn from(data: [f32; 3]) -> Self {
        Self {
            0: image::Rgb { 0: data },
        }
    }

    #[inline]
    pub fn into_raw(&self) -> [u8; 4] {
        [
            (self.r() * 255.0) as u8,
            (self.g() * 255.0) as u8,
            (self.b() * 255.0) as u8,
            255,
        ]
    }

    #[inline]
    pub fn data(&self) -> [f32; 3] {
        self.0 .0
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.data()[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.data()[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.data()[2]
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Color::from(self.data().map(|channel| channel * rhs))
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Color::from([self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b()])
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Color::from([self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b()])
    }
}
