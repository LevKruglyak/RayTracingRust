use cgmath::{InnerSpace, Vector2};
use std::f32::consts::PI;

use crate::{color::Color, ray::Ray, utils::to_spherical_coords};

pub trait Background: Sync {
    fn sample(&self, ray: &Ray) -> Color;
}

pub struct UniformBackground {
    color: Color,
}

impl UniformBackground {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Background for UniformBackground {
    fn sample(&self, _: &Ray) -> Color {
        self.color
    }
}

pub struct GradientBackground {
    up: Color,
    down: Color,
}

impl GradientBackground {
    pub fn new(up: Color, down: Color) -> Self {
        Self { up, down }
    }
}

impl Background for GradientBackground {
    fn sample(&self, ray: &Ray) -> Color {
        ray.vertical_grad(self.up, self.down)
    }
}

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
                skymap.image[position.x() + position.y() * skymap.width] = Color::new(r, g, b);
            },
        )
        .expect("could not read image!");
        println!("loaded '{}'", path);
        image.layer_data.channel_data.pixels
    }
}

impl Background for SkyMap {
    fn sample(&self, ray: &Ray) -> Color {
        let spherical_coords = to_spherical_coords(ray.direction.normalize());
        let u = spherical_coords.x / PI;
        let v = spherical_coords.y / (2.0 * PI);
        let x = (v * self.width as f32) as usize % self.width;
        let y = (u * self.height as f32) as usize % self.height;
        self.image[x + y * self.width]
    }
}