use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
    pub fn from(data: [f32; 3]) -> Self {
        Self {
            r: data[0],
            g: data[1],
            b: data[2],
        }
    }

    #[inline]
    pub fn into_raw(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            255,
        ]
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            b: self.b * rhs,
            g: self.g * rhs,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
