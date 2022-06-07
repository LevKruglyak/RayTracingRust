use palette::{Pixel, Srgba};

pub struct RayTracingDemo {
    width: u32,
    height: u32,
    pixels: Vec<Srgba>,
    pub needs_redraw: bool,
}

impl RayTracingDemo {
    pub fn new(width: u32, height: u32, background: Srgba) -> Self {
        Self {
            width,
            height,
            pixels: vec![background; (width * height) as usize],
            needs_redraw: true,
        }
    }

    pub fn update(&mut self, param: f32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let xf = (x as f32) / (self.width as f32);
                let yf = (y as f32) / (self.height as f32);
                self.pixels[(x + y * self.width) as usize] = Srgba::new(xf, yf, param, 1.0);
            }
        }

        self.needs_redraw = true;
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        self.needs_redraw = false;

        assert_eq!(self.pixels.len() * 4, frame.len());
        for (pixel, result) in self.pixels.iter().zip(frame.chunks_exact_mut(4)) {
            let pixel: [u8; 4] = Srgba::into_raw(pixel.into_format());
            result.copy_from_slice(&pixel);
        }
    }
}
