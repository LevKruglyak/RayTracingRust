use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};

use crate::{
    core::traits::Background,
    utils::{
        color::Color,
        math::to_spherical_coords,
        ray::Ray,
        types::{Float, PI},
    },
};

#[derive(Serialize, Deserialize)]
pub struct UniformBackground {
    pub color: Color,
}

impl UniformBackground {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

#[typetag::serde]
impl Background for UniformBackground {
    fn sample(&self, _: &Ray) -> Color {
        self.color
    }
}

#[derive(Serialize, Deserialize)]
pub struct GradientBackground {
    pub top: Color,
    pub bottom: Color,
}

impl GradientBackground {
    pub fn new(top: Color, bottom: Color) -> Self {
        Self { top, bottom }
    }
}

#[typetag::serde]
impl Background for GradientBackground {
    fn sample(&self, ray: &Ray) -> Color {
        ray.vertical_grad(self.top, self.bottom)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SkyMap {
    image: Vec<Color>,
    width: usize,
    height: usize,
}

impl SkyMap {
    pub fn new(path: &str) -> Self {
        let image = exr::prelude::read_first_rgba_layer_from_file(
            path,
            |resolution, _| Self {
                image: vec![Color::default(); resolution.width() * resolution.height()],
                width: resolution.width(),
                height: resolution.height(),
            },
            |skymap: &mut Self, position, (r, g, b, _): (f32, f32, f32, f32)| {
                skymap.image[position.x() + position.y() * skymap.width] =
                    Color::new(r as Float, g as Float, b as Float);
            },
        )
        .expect("could not read image!");
        println!(
            "loaded '{}', dimensions: {},{}",
            path,
            image.layer_data.channel_data.pixels.width,
            image.layer_data.channel_data.pixels.height
        );
        image.layer_data.channel_data.pixels
    }
}

#[typetag::serde]
impl Background for SkyMap {
    fn sample(&self, ray: &Ray) -> Color {
        let spherical_coords = to_spherical_coords(ray.direction.normalize());
        let u = spherical_coords.x / PI;
        let v = spherical_coords.y / (2.0 * PI);
        let x = (v * self.width as Float) as usize % self.width;
        let y = self.height - 1 - (u * self.height as Float) as usize % self.height;
        self.image[x + y * self.width]
    }
}
