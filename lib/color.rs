use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

use crate::utils::types::Float;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Color {
    #[inline]
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn from(data: [Float; 3]) -> Self {
        Self {
            r: data[0],
            g: data[1],
            b: data[2],
        }
    }

    #[inline]
    pub fn data(&self) -> [Float; 3] {
        [self.r, self.g, self.b]
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

impl Mul<Float> for Color {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
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
