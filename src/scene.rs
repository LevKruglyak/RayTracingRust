use cgmath::Vector3;

pub struct Camera {
    pub focal_length: f32,
    pub origin: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
}

impl Camera {
    fn new(viewport_width: f32, viewport_height: f32) -> Self {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);

        let default_focal_length = 1.0;

        Self {
            focal_length: default_focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vector3::new(0.0, 0.0, default_focal_length),
        }
    }
}

pub struct Scene {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Self {
        let viewport_width = 2.0;
        let viewport_height = 2.0 * (height as f32) / (width as f32);

        Self {
            viewport_width,
            viewport_height,
            camera: Camera::new(viewport_width, viewport_height),
        }
    }
}
