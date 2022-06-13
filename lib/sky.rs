use crate::{ray::Ray, color::Color};

pub trait Background {
    fn sample(&self, ray: &Ray) -> Color;
}

pub struct UniformBackground {
    color: Color
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
        Self { up, down, }
    }
}

impl Background for GradientBackground {
    fn sample(&self, ray: &Ray) -> Color {
        ray.vertical_grad(self.up, self.down)
    }
}

// pub struct SkyMap {
//     image: Vec<Color>,
// }
